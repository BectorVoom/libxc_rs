//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta469 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1828;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1829;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1830;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta469<F: Float>(t210: F, t6679: F, t3139: F, t6717: F, t3113: F, t6754: F, t3107: F, t6753: F, t1012: F, t1933: F, t607: F, t1937: F, t1000: F, t1025: F, t23414: F, t23419: F, t3073: F, t3098: F, t3123: F, t3143: F, t3148: F, t6755: F, t6765: F) -> (F, F, F, F, F, F, F) {
        let t23422 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1828::<F>(t210, t6679);
        let (t23425, t23433) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1829::<F>(t3139, t6717, t3113, t6754);
        let (t23436, t23437) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1830::<F>(t3107, t6753, t1012);
        let (t23443, t23445) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1831::<F>(t1933, t607, t1937, t1000, t1025, t23414, t23419, t23422, t23425, t23433, t23437, t3073, t3098, t3123, t3143, t3148, t6717, t6755, t6765);
    (t23422, t23425, t23433, t23436, t23437, t23443, t23445)
}
