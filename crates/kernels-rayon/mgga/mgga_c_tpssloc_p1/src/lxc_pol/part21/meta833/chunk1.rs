//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2942/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2942(t10255: f64, t17800: f64, t17804: f64, t2986: f64, t42830: f64, t42962: f64, t42968: f64, t4510: f64, t5821: f64, t59715: f64, t61245: f64, t61252: f64, t61258: f64, t61261: f64, t61264: f64, t61273: f64) -> f64 {
    let t61275 = 0.24691358024691358024e-3_f64 * t61245 + 0.55555555555555555554e-3_f64 * t2986 * t17804 * t10255 - 0.18518518518518518518e-3_f64 * t61252 + 0.55555555555555555554e-3_f64 * t2986 * t17800 * t10255 + 0.49382716049382716048e-3_f64 * t61258 + 0.5761316872427983539e-3_f64 * t61261 - 0.24691358024691358024e-3_f64 * t61264 - 0.22222222222222222221e-2_f64 * t2986 * t4510 * t59715 + 0.12345679012345679012e-3_f64 * t42962 + 0.32921810699588477366e-3_f64 * t42968 - 0.54320987654320987651e-2_f64 * t42830 * t5821 + 0.98765432098765432095e-3_f64 * t61273;
    t61275
}
