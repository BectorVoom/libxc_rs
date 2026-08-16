//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta185 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk815;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk816;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk817;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk818;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta185(t3062: f64, t820: f64, t10402: f64, t3200: f64, t3051: f64, t121: f64, t3061: f64, t1008: f64, t349: f64, t1011: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t10408 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk815(t3062, t820);
        let t10413 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk816(t10402, t3200);
        let t10422 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk817(t3051, t820);
        let (t10457, t10468, t10469, t10470, t10471) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk818(t121, t3061, t1008, t349, t1011);
    (t10408, t10413, t10422, t10457, t10468, t10469, t10470, t10471)
}
