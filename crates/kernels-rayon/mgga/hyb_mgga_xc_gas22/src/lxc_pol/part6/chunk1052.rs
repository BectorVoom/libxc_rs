//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1052/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1052(t3827: f64, t600: f64, t3844: f64, t60: f64, t604: f64, t63: f64, t608: f64, t66: f64, t612: f64, t69: f64, t6004: f64, t1941: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9954 = t600 * t3827;
    let t9959 = t60 * t3844;
    let t9962 = t604 * t3827;
    let t9967 = t63 * t3844;
    let t9970 = t608 * t3827;
    let t9975 = t66 * t3844;
    let t9978 = t612 * t3827;
    let t9983 = t69 * t3844;
    let t9986 = t6004 * t3827;
    let t9991 = t1941 * t3844;
    (t9954, t9959, t9962, t9967, t9970, t9975, t9978, t9983, t9986, t9991)
}
