//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta665 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2214;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2215;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2216;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2217;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2218;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2219;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta665(t2394: f64, t5682: f64, t5686: f64, t17158: f64, t690: f64, t17162: f64, t17153: f64, t17168: f64, t17172: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t59688 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2214(t2394, t5682);
        let t59694 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2215(t2394, t5686);
        let t59700 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2216(t17158, t690);
        let t59702 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2217(t17162, t690);
        let t59704 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2218(t17153, t690);
        let t59759 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2219(t17168, t690);
        let t59761 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2220(t17172, t690);
    (t59688, t59694, t59700, t59702, t59704, t59759, t59761)
}
