//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta595 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2024;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta595(t22715: f64, t6551: f64, t6640: f64, t117: f64, t4179: f64, t6559: f64, t22893: f64, t23036: f64, t229: f64, t268: f64, t22988: f64, t23110: f64, t23154: f64, t23164: f64, t234: f64, t2710: f64, t23176: f64, t23185: f64, t131: f64, t2587: f64, t81142: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81632, t81633, t81640, t81642, t81651) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2024(t22715, t6551, t6640, t117, t4179, t6559, t22893, t23036, t229, t268);
        let (t81653, t81656, t81658, t81670, t81686) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2025(t22988, t23110, t81651, t22893, t23154, t23164, t234, t2710, t23176, t23185, t131, t2587, t81142);
    (t81632, t81633, t81640, t81642, t81651, t81653, t81656, t81658, t81670, t81686)
}
