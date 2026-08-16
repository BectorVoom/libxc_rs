//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1469/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1469(t121179: f64, t121181: f64, t121184: f64, t121190: f64, t121192: f64, t121194: f64, t122920: f64, t124863: f64, t2040: f64, t2075: f64, t27226: f64, t27371: f64, t27888: f64, t33690: f64, t510: f64, t7050: f64, t7266: f64, t7802: f64, t8329: f64) -> f64 {
    let t124947 = -2.0_f64 * t122920 * t2040 - t124863 * t510 - t2075 * t27371 - 2.0_f64 * t27226 * t7266 - 2.0_f64 * t27888 * t7802 - 2.0_f64 * t33690 * t7050 - t121179 - t121181 + t121184 - t121190 - t121192 - t121194 - t8329;
    t124947
}
