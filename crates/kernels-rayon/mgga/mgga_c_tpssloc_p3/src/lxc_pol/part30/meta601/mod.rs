//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta601 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1989;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1990;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta601(t225: f64, t814: f64, t6648: f64, t81612: f64, t22715: f64, t6551: f64, t6640: f64, t117: f64, t4179: f64, t6559: f64, t229: f64, t268: f64, t2627: f64, t6624: f64, t131: f64, t2587: f64, t81142: f64, t1905: f64, t9537: f64, t81151: f64, t23172: f64, t133: f64, t1891: f64, t6601: f64, t80953: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81613, t81615, t81632, t81633, t81640, t81651) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1989(t225, t814, t6648, t81612, t22715, t6551, t6640, t117, t4179, t6559, t229, t268);
        let (t81679, t81686, t81689, t81715, t81717, t81735) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1990(t2627, t6624, t131, t2587, t81142, t1905, t9537, t81151, t23172, t133, t1891, t6601, t80953);
    (t81613, t81615, t81632, t81633, t81640, t81651, t81679, t81686, t81689, t81715, t81717, t81735)
}
