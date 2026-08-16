//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta458 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1804;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1805;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta458(t22690: f64, t6638: f64, t23171: f64, t828: f64, t852: f64, t232: f64, t6646: f64, t1888: f64, t10097: f64, t206: f64, t268: f64, t6559: f64, t23110: f64, t6648: f64, t226: f64, t23026: f64, t23029: f64, t23032: f64, t23038: f64, t23151: f64, t23156: f64, t23160: f64, t23167: f64, t23170: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23172, t23174, t23176, t23177, t23178, t23180, t23181, t23182, t23185) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1804(t22690, t6638, t23171, t828, t852, t232, t6646, t1888, t10097, t206, t268, t6559);
        let (t23186, t23187, t23189) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1805(t23110, t6648, t23185, t226, t23026, t23029, t23032, t23038, t23151, t23156, t23160, t23167, t23170, t23174, t23178, t23182);
    (t23172, t23174, t23176, t23177, t23180, t23181, t23185, t23186, t23187, t23189)
}
