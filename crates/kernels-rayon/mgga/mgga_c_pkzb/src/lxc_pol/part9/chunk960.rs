//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 960/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk960(t237: f64, t7266: f64, t7306: f64, t7418: f64, t7521: f64, t1991: f64, t2860: f64, t1954: f64, t2848: f64, t723: f64, t730: f64, t1107: f64, t5498: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7524 = t237 * (t7266 + t7306 + t7418 + t7521);
    let t7526 = 0.11696447245269292414e1_f64 * t2860 * t1991;
    let t7527 = t1954 * t2848;
    let t7528 = t7527 * t723;
    let t7530 = 0.23392894490538584828e1_f64 * t730 * t7528;
    let t7531 = t5498 * t1107;
    (t7524, t7526, t7527, t7528, t7530, t7531)
}
