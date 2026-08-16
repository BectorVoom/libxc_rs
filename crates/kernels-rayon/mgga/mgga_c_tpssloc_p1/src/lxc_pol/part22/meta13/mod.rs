//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta13 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk102;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk103;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk104;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk105;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk106;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk107;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk108;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk109;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta13(t202: f64, t68: f64, t228: f64, t59: f64, t226: f64, t15: f64, t61: f64, t154: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t229, t230) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk102(t202, t68);
        let t232 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk103(t228, t230);
        let (t233, t234, t235) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk104(t232, t68);
        let t236 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk105(t59);
        let t237 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk106(t235, t236);
        let t238 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk107(t226, t237);
        let t240 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk108(t15, t61);
        let t241 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk109(t154);
        let t242 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk110(t240, t241);
    (t229, t230, t232, t233, t234, t235, t236, t237, t238, t240, t241, t242)
}
