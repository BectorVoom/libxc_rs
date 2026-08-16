//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta109 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk582;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk583;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk584;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta109(t118: f64, t1484: f64, t794: f64, t2576: f64, t1493: f64, t225: f64, t1496: f64, t2563: f64, t1499: f64, t68: f64, t816: f64, t1500: f64, t838: f64, t842: f64, t242: f64, t2628: f64, t812: f64, t244: f64, t67: f64, t246: f64, t120: f64, t1509: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4134, t4135, t4147, t4152, t4166) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk582(t118, t1484, t794, t2576, t1493, t225, t1496, t2563, t1499, t68);
        let (t4167, t4170, t4172, t4177, t4178) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk583(t4166, t816, t1500, t838, t842, t242, t2628, t812);
        let t4180 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk584(t244, t67, t246);
        let t4181 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk585(t120, t1509);
    (t4134, t4135, t4147, t4152, t4166, t4167, t4170, t4172, t4177, t4178, t4180, t4181)
}
