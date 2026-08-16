//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1427/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1427(t2851: f64, t30891: f64, t1123: f64, t3951: f64, t1129: f64, t4489: f64, t30763: f64, t3740: f64, t4535: f64, t1118: f64, t11410: f64, t30776: f64) -> (f64, f64, f64, f64, f64) {
    let t30903 = t2851 * t30891;
    let t30906 = t3951 * t1123;
    let t30908 = t4489 * t30906 * t1129;
    let t30915 = t3740 * t30763;
    let t30918 = t4535 * t1129;
    let t30919 = t1118 * t30918;
    let t30922 = t11410 * t30776;
    (t30903, t30908, t30915, t30919, t30922)
}
