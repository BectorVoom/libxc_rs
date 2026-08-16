//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1318/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1318(t22574: f64, t25988: f64, t36363: f64, t31044: f64, t7685: f64, t26168: f64, t8450: f64, t24991: f64, t19577: f64, t24995: f64, t37589: f64, t5308: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120083 = 3.0_f64 * t22574 * t36363 * t25988;
    let t120085 = 2.0_f64 * t7685 * t31044;
    let t120086 = t8450 * t26168;
    let t120088 = t8450 * t24991;
    let t120092 = 3.0_f64 * t22574 * t36363 * t19577;
    let t120095 = 6.0_f64 * t24995 * t37589 * t5308;
    (t120083, t120085, t120086, t120088, t120092, t120095)
}
