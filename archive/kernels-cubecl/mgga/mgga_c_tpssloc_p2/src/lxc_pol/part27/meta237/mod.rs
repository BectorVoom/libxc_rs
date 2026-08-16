//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1134;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1135;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1136;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1137;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1138;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1139;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta237<F: Float>(t641: F, t71: F, t1863: F, t5: F, t1860: F, t1865: F, t6486: F, t6490: F, t6492: F, t6495: F, t6506: F, t112: F, t111: F, t1868: F, t1874: F, t2314: F, t4034: F, t1266: F, t1873: F, t652: F, t107: F, t625: F, t63: F, t656: F, t109: F, t666: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t6509 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1134::<F>(t641, t71);
        let t6510 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1135::<F>(t1863, t6509);
        let (t6514, t6515) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1136::<F>(t5, t1860, t1865, t6486, t6490, t6492, t6495, t6506, t6510, t112);
        let t6517 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1137::<F>(t111, t1868);
        let (t6522, t6524, t6525) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1138::<F>(t1874, t2314, t4034, t1266, t1873);
        let (t6527, t6529, t6530) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1139::<F>(t652, t6525, t107, t625, t63, t656);
        let t6534 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1140::<F>(t109, t6530, t666, t6529);
    (t6509, t6510, t6514, t6515, t6517, t6522, t6524, t6525, t6527, t6529, t6530, t6534)
}
