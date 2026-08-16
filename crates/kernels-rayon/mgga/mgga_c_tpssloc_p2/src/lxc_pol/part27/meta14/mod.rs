//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta14 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk103;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk104;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk105;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk106;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk107;
use chunk5::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk108;
use chunk6::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk109;
use chunk7::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk110;
use chunk8::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk111;
use chunk9::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk112;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta14(t234: f64, t68: f64, t59: f64, t226: f64, t15: f64, t61: f64, t154: f64, t201: f64, t132: f64, t67: f64, t120: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t235 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk103(t234, t68);
        let t236 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk104(t59);
        let t237 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk105(t235, t236);
        let (t238, t240) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk106(t226, t237, t15, t61);
        let t241 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk107(t154);
        let t242 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk108(t240, t241);
        let t243 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk109(t201);
        let t244 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk110(t243);
        let t246 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk111(t132);
        let (t247, t248) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk112(t246, t67, t120);
    (t235, t236, t237, t238, t240, t241, t242, t243, t244, t246, t247, t248)
}
