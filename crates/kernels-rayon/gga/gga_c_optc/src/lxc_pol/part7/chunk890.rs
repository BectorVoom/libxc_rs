//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 890/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk890(t2993: f64, t8565: f64, t1055: f64, t3012: f64, t3020: f64, t3018: f64, t1036: f64, t2985: f64, t1057: f64, t2988: f64, t3013: f64, t1032: f64, t3017: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8567 = 6.0_f64 * t2993 * t8565;
    let t8569 = t3012 * t3020 * t1055;
    let t8571 = 0.48245472966453314466e2_f64 * t3018 * t8569;
    let t8572 = t2985 * t1036;
    let t8574 = 3.0_f64 * t8572 * t1057;
    let t8576 = 3.0_f64 * t2988 * t3013;
    let t8577 = t1032 * t3017;
    (t8567, t8569, t8571, t8572, t8574, t8576, t8577)
}
