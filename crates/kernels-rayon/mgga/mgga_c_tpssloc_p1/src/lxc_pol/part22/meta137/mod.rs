//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta137 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk886;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk887;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk888;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk889;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk890;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk891;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk892;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk893;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk894;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta137(t242: f64, t2628: f64, t812: f64, t244: f64, t67: f64, t246: f64, t120: f64, t1509: f64, t2632: f64, t828: f64, t1512: f64, t2639: f64, t249: f64, t2571: f64, t2602: f64, t2603: f64, t2618: f64, t4152: f64, t4155: f64, t4159: f64, t4163: f64, t4167: f64, t4170: f64, t4172: f64, t787: f64, t831: f64, t849: f64, t2645: f64, t2647: f64, t157: f64, t2658: f64, t1409: f64, t184: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4177, t4178) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk886(t242, t2628, t812);
        let (t4179, t4180) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk887(t244, t67, t246);
        let t4181 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk888(t120, t1509);
        let t4182 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk889(t2632, t828);
        let t4184 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk890(t4180, t4181, t4182);
        let (t4187, t4189) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk891(t1512, t2639, t249, t2571, t2602, t2603, t2618, t4152, t4155, t4159, t4163, t4167, t4170, t4172, t4178, t4184, t787, t831, t849);
        let t4191 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk892(t2645, t2647, t4181);
        let t4194 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk893(t157, t2658);
        let t4195 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk894(t1409, t184);
    (t4177, t4178, t4179, t4180, t4181, t4182, t4184, t4187, t4189, t4191, t4194, t4195)
}
