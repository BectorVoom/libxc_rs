//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1406/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1406(t27082: f64, t496: f64, t3274: f64, t9260: f64, t1220: f64, t1222: f64, t3902: f64, t3280: f64, t7274: f64, t3285: f64, t2908: f64, t2910: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28010 = t27082 * t496;
    let t28017 = t3274 * t9260;
    let t28020 = t1220 * t3902 * t1222;
    let t28023 = t1220 * t7274 * t3280;
    let t28026 = t1220 * t7274 * t3285;
    let t28028 = t2908 * t2908;
    let t28030 = t2910 * t2910;
    (t28010, t28017, t28020, t28023, t28026, t28028, t28030)
}
