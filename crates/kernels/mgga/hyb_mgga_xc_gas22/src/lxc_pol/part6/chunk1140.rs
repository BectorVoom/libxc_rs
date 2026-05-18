//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1140/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1140<F: Float>(t1159: F, t4530: F, t4540: F, t2824: F, t5204: F, t537: F, t1539: F, t2857: F, t3742: F, t1535: F, t2850: F, t1118: F, sigma2: F, tau1: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11315 = t1159 * t4530;
    let t11319 = t1159 * t4540;
    let t11320 = t11319 * t2824;
    let t11329 = t5204 * t537;
    let t11335 = t1539 * t2857 * sigma2;
    let t11336 = t11335 * t3742;
    let t11342 = t1535 * t2850 * sigma2;
    let t11343 = t11342 * t3742;
    let t11346 = t1118 * tau1;
    (t11315, t11319, t11320, t11329, t11335, t11336, t11342, t11343, t11346)
}
