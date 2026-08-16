//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk966;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk967;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta187(t1519: f64, t798: f64, t1496: f64, t2563: f64, t1495: f64, t210: f64, t776: f64, t119: f64, t4119: f64, t225: f64, t4142: f64, t237: f64, t1499: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4149, t4152, t4155, t4158, t4159, t4162) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk966(t1519, t798, t1496, t2563, t1495, t210, t776, t119, t4119, t225, t4142);
        let (t4163, t4166) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk967(t237, t4162, t1499, t68);
    (t4149, t4152, t4155, t4158, t4159, t4162, t4163, t4166)
}
