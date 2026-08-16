//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1273/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1273(t3013: f64, t8572: f64, t2985: f64, t3017: f64, t3021: f64, t2988: f64, t8680: f64, t1032: f64, t8685: f64, t8689: f64, t2991: f64, t3016: f64, t375: f64) -> (f64, f64, f64, f64, f64) {
    let t26237 = 6.0_f64 * t8572 * t3013;
    let t26238 = t2985 * t3017;
    let t26240 = 0.96490945932906628932e2_f64 * t26238 * t3021;
    let t26242 = 4.0_f64 * t2988 * t8680;
    let t26243 = t1032 * t8685;
    let t26245 = 0.20690005882282467367e4_f64 * t26243 * t8689;
    let t26248 = t375 / t3016 / t2991;
    (t26237, t26240, t26242, t26245, t26248)
}
