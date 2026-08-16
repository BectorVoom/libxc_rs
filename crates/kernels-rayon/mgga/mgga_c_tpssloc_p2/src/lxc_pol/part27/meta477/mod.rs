//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta477 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1848;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta477(t2771: f64, t6690: f64, t23593: f64, t3034: f64, t38: f64, t131: f64, t350: f64, t3030: f64, t344: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t23594, t23595, t23598, t23599, t23600, t23601) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1848(t2771, t6690, t23593, t3034, t38, t131, t350);
        let t23602 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1849(t3030, t344);
    (t23594, t23595, t23598, t23599, t23600, t23601, t23602)
}
