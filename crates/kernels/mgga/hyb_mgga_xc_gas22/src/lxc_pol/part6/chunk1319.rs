//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1319/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1319<F: Float>(t2851: F, t30891: F, t1123: F, t3951: F, t1129: F, t4489: F, t30763: F, t3740: F, t4535: F, t1118: F, t11410: F, t30776: F, t22705: F, t412: F, t4576: F, t9691: F) -> (F, F, F, F, F, F, F) {
    let t30903 = t2851 * t30891;
    let t30906 = t3951 * t1123;
    let t30908 = t4489 * t30906 * t1129;
    let t30915 = t3740 * t30763;
    let t30918 = t4535 * t1129;
    let t30919 = t1118 * t30918;
    let t30922 = t11410 * t30776;
    let t30930 = t22705 * t412 * t30776;
    let t30933 = t4576 * t9691;
    (t30903, t30908, t30915, t30919, t30922, t30930, t30933)
}
