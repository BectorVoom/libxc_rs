//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 994/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk994(t106: f64, t1147: f64, t12522: f64, t1550: f64, t15706: f64, t17947: f64, t17960: f64, t17964: f64, t18174: f64, t4403: f64, t4410: f64, t470: f64, t5351: f64, t5430: f64, t8997: f64) -> f64 {
    let t18178 = 0.27818116767324025134e1_f64 * t106 * t17947 * t470 - 0.83454350301972075402e1_f64 * t106 * t15706 * t1550 + 0.16690870060394415081e2_f64 * t106 * t12522 * t5351 - 0.83454350301972075402e1_f64 * t106 * t4403 * t5430 - 0.1669087006039441508e2_f64 * t106 * t8997 * t17960 + 0.16690870060394415081e2_f64 * t4410 * t17964 - 0.27818116767324025134e1_f64 * t106 * t1147 * t18174;
    t18178
}
