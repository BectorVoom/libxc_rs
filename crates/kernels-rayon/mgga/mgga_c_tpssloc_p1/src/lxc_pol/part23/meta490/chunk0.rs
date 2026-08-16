//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1498/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1498(t25: f64, t17: f64, t184: f64, t79888: f64, t57208: f64, t6463: f64, t57211: f64, t54451: f64, t74496: f64, t1298: f64, t19606: f64, t20216: f64, t3704: f64, t39861: f64, t5170: f64, t5397: f64, t75911: f64, t79859: f64, t79864: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t79942 = t17 * t79888 * t184;
    let t79946 = 48.0_f64 * t57208;
    let t79947 = t6463 * t6463;
    let t79952 = 0.14649157844805236043e-2_f64 * t57211;
    let t79953 = 0.4155806185363551302e3_f64 * t54451;
    let t79954 = 4.0_f64 * t74496;
    let t79970 = piecewise3(t26, 0.0_f64, -56.0_f64 / 81.0_f64 * t39861 * t79859 + 16.0_f64 / 9.0_f64 * t19606 * t5397 - 2.0_f64 / 3.0_f64 * t3704 * t79864 - 8.0_f64 / 9.0_f64 * t5170 * t20216 + 2.0_f64 / 3.0_f64 * t1298 * t75911);
    (t79942, t79946, t79947, t79952, t79953, t79954, t79970)
}
