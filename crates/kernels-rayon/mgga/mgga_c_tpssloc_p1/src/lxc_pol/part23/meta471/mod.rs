//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta471 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1402;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1403;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1404;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1405;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1406;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1407;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta471(t11147: f64, t75836: f64, t136: f64, t3297: f64, t11153: f64, t1113: f64, t1089: f64, t75912: f64, t1088: f64, t123: f64, t43809: f64, t77961: f64, t77969: f64, t50834: f64, t71335: f64, t71337: f64, t77959: f64, t77963: f64, t77967: f64, t77971: f64, t11145: f64, t77957: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t77973, t77975, t77977, t77979, t77981, t77983, t77989) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1402(t11147, t75836, t136, t3297, t11153, t1113, t1089, t75912, t1088, t123);
        let t77992 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1403(t1088, t123, t77981);
        let t77995 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1404(t123, t43809, t77961);
        let t77998 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1405(t1088, t123, t77969);
        let t78000 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1406(t50834, t71335, t71337, t77959, t77963, t77967, t77971, t77975, t77979, t77983, t77989, t77992, t77995, t77998);
        let t78002 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1407(t11145, t123, t77957);
    (t77973, t77975, t77977, t77979, t77981, t77983, t77989, t77992, t77995, t77998, t78000, t78002)
}
