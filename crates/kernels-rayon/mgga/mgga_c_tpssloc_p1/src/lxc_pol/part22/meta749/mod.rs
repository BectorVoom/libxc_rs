//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta749 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2503;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2504;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2505;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2506;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2507;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2508;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2509;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2510;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta749(t47774: f64, t50992: f64, t68513: f64, t20234: f64, t43791: f64, t607: f64, t11145: f64, t123: f64, t20217: f64, t3242: f64, t3240: f64, t21766: f64, t690: f64, t21773: f64, t21759: f64, t1089: f64, t67060: f64, t1088: f64, t21770: f64, t21777: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t71130 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2503(t47774, t50992, t68513);
        let (t71133, t71135) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2504(t20234, t43791, t607, t11145, t123);
        let (t71138, t71140) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2505(t20217, t3242, t607, t123, t3240);
        let t71142 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2506(t21766, t690);
        let t71144 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2507(t21773, t690);
        let t71146 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2508(t21759, t690);
        let (t71148, t71150) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2509(t1089, t67060, t1088, t123);
        let t71152 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2510(t21770, t690);
        let t71154 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2511(t21777, t690);
    (t71130, t71133, t71135, t71138, t71140, t71142, t71144, t71146, t71148, t71150, t71152, t71154)
}
