//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta237 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1134;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1135;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1136;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1137;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1138;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1139;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta237(t641: f64, t71: f64, t1863: f64, t5: f64, t1860: f64, t1865: f64, t6486: f64, t6490: f64, t6492: f64, t6495: f64, t6506: f64, t112: f64, t111: f64, t1868: f64, t1874: f64, t2314: f64, t4034: f64, t1266: f64, t1873: f64, t652: f64, t107: f64, t625: f64, t63: f64, t656: f64, t109: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6509 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1134(t641, t71);
        let t6510 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1135(t1863, t6509);
        let (t6514, t6515) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1136(t5, t1860, t1865, t6486, t6490, t6492, t6495, t6506, t6510, t112);
        let t6517 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1137(t111, t1868);
        let (t6522, t6524, t6525) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1138(t1874, t2314, t4034, t1266, t1873);
        let (t6527, t6529, t6530) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1139(t652, t6525, t107, t625, t63, t656);
        let t6534 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1140(t109, t6530, t666, t6529);
    (t6509, t6510, t6514, t6515, t6517, t6522, t6524, t6525, t6527, t6529, t6530, t6534)
}
