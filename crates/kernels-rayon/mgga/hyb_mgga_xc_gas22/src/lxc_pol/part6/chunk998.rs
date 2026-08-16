//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 998/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk998(t2594: f64, t3596: f64, t3604: f64, t7165: f64, t2598: f64, t3579: f64, t3605: f64, t260: f64, t3557: f64, t1006: f64, t9195: f64, t997: f64) -> (f64, f64, f64, f64, f64) {
    let t9282 = t3596 * t2594;
    let t9285 = t3604 * t7165;
    let t9288 = t2598 * t3579;
    let t9289 = t9288 * t3605;
    let t9296 = t260 * t3557;
    let t9306 = t997 * t9195 * t1006;
    (t9282, t9285, t9289, t9296, t9306)
}
