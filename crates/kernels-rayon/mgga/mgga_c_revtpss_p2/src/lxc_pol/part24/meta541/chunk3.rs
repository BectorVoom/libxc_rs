//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1592/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1592(t114: f64, t87050: f64, t5876: f64, t5883: f64, t1519: f64, t18245: f64, t1843: f64, t22578: f64, t22633: f64, t22634: f64, t22639: f64, t30138: f64, t4248: f64, t508: f64, t5884: f64, t5887: f64, t5920: f64, t5921: f64, t651: f64, t6765: f64, t75941: f64, t7732: f64) -> (f64, f64, f64) {
    let t115 = 1.0_f64 < t114;
    let t87051 = piecewise3(t115, 0.0_f64, t87050);
    let t87064 = t5876 * t5883;
    let t87071 = -8.0_f64 * t1843 * t22633 * t651 - 2.0_f64 * t508 * t651 * t87051 - 12.0_f64 * t5920 * t651 * t6765 - 8.0_f64 * t1519 * t75941 - 24.0_f64 * t18245 * t5887 - 24.0_f64 * t1843 * t22639 - 24.0_f64 * t22578 * t4248 - 24.0_f64 * t22578 * t7732 - 8.0_f64 * t22634 * t4248 - 8.0_f64 * t22634 * t7732 - 24.0_f64 * t30138 * t5921 - 12.0_f64 * t508 * t87064 - 12.0_f64 * t5884 * t6765;
    (t87051, t87064, t87071)
}
