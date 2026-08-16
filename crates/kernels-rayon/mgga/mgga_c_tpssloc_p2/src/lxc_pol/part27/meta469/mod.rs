//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1828;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1829;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1830;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta469(t210: f64, t6679: f64, t3139: f64, t6717: f64, t3113: f64, t6754: f64, t3107: f64, t6753: f64, t1012: f64, t1933: f64, t607: f64, t1937: f64, t1000: f64, t1025: f64, t23414: f64, t23419: f64, t3073: f64, t3098: f64, t3123: f64, t3143: f64, t3148: f64, t6755: f64, t6765: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t23422 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1828(t210, t6679);
        let (t23425, t23433) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1829(t3139, t6717, t3113, t6754);
        let (t23436, t23437) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1830(t3107, t6753, t1012);
        let (t23443, t23445) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1831(t1933, t607, t1937, t1000, t1025, t23414, t23419, t23422, t23425, t23433, t23437, t3073, t3098, t3123, t3143, t3148, t6717, t6755, t6765);
    (t23422, t23425, t23433, t23436, t23437, t23443, t23445)
}
