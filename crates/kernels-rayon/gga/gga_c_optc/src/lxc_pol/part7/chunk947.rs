//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 947/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk947(t1170: f64, t9034: f64, t1128: f64, t3188: f64, t3186: f64, t2856: f64, t3236: f64, t3235: f64, t1900: f64, t553: f64) -> (f64, f64, f64, f64, f64) {
    let t9035 = t1170 * t9034;
    let t9037 = t1128 * t3188;
    let t9038 = t3186 * t9037;
    let t9040 = t2856 * t3236;
    let t9041 = t3235 * t9040;
    let t9044 = t553 * t1900;
    (t9035, t9038, t9040, t9041, t9044)
}
