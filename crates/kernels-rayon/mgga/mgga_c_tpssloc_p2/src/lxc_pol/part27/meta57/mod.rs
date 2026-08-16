//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta57 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk398;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk399;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk400;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk401;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk402;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta57(t1088: f64, t1090: f64, t123: f64, t1087: f64, t423: f64, t419: f64, t409: f64, t410: f64, t1086: f64, t407: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1091, t1092, t1094, t1096, t1097, t1098) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk398(t1088, t1090, t123, t1087, t423, t419);
        let t1099 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk399(t1098, t409);
        let t1100 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk400(t410);
        let t1102 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk401(t1086, t1092);
        let (t1103, t1105, t1107) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk402(t1100, t1102, t1086, t407);
    (t1091, t1092, t1094, t1096, t1097, t1098, t1099, t1100, t1102, t1103, t1105, t1107)
}
