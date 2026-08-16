//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2737/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2737(t1390: f64, t16497: f64, t193: f64, t3918: f64, t39595: f64, t39615: f64, t5187: f64, t533: f64, t56411: f64, t56412: f64, t56416: f64, t56417: f64, t56457: f64, t56605: f64, t56649: f64, t57203: f64, t57204: f64, t57205: f64, t57795: f64) -> f64 {
    let t57801 = t39595 + t56411 - t56412 + 12.0_f64 * t3918 * t16497 * t5187 + t56416 - t56417 + t193 * t533 * (t56457 + t56605 + t56649 + t57795) * t1390 - t57203 - t57204 - t57205 + t39615;
    t57801
}
