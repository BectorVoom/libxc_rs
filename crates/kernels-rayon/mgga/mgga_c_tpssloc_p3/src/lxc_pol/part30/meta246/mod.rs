//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1110;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1111;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1112;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1113;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1114;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta246(t652: f64, t6525: f64, t107: f64, t625: f64, t63: f64, t656: f64, t109: f64, t666: f64, t510: f64, t1976: f64, t671: f64, t25: f64, t776: f64, t154: f64, t781: f64, t1879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6527, t6529, t6530) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1110(t652, t6525, t107, t625, t63, t656);
        let t6534 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1111(t109, t6530, t666, t6529);
        let t6535 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1112(t510, t6534);
        let (t6537, t6539, t6542, t6546) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1113(t652, t6535, t1976, t671, t25, t776, t154, t781);
        let t6547 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1114(t1879, t6546);
    (t6527, t6529, t6530, t6534, t6535, t6537, t6539, t6542, t6546, t6547)
}
