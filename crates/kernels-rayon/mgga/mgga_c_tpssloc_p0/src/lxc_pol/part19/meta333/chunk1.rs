//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1195/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1195(t40722: f64, t2523: f64, t39400: f64, t39408: f64, t39411: f64, t39463: f64, t39468: f64, t39472: f64, t39476: f64, t40708: f64, t40711: f64, t40714: f64, t40716: f64, t40721: f64, t4314: f64, t9616: f64) -> (f64, f64) {
    let t40723 = 0.22787578869697033845e-2_f64 * t40722;
    let t40724 = 72.0_f64 * t2523 * t4314 * t9616 - t39400 + t39408 + t39411 + t39463 - t39468 - t39472 - t39476 + t40708 + t40711 - t40714 + t40716 - t40721 - t40723;
    (t40723, t40724)
}
