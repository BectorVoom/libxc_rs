//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1024/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1024(t209: f64, t60: f64, t6472: f64, t6519: f64, t6348: f64, t6500: f64, t1772: f64, t1796: f64, t1990: f64, t1867: f64, t22120: f64, t601: f64, t6424: f64) -> (f64, f64, f64, f64) {
    let t22287 = t60 * t209;
    let t22290 = 0.13012297059337829057e0_f64 * t22287 * t6519 * t6472;
    let t22293 = 0.1926377843805564792e1_f64 * t22287 * t6500 * t6348;
    let t22296 = 0.86748647062252193713e-1_f64 * t1796 * t1772 * t1990;
    let t22300 = 0.6233672123775310788e3_f64 * t601 * t6424 * t22120 * t1867;
    (t22290, t22293, t22296, t22300)
}
