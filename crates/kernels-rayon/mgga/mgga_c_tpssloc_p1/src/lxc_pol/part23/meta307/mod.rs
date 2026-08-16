//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta307 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1046;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1047;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1048;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1049;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1050;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1051;
use chunk6::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1052;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta307(t1113: f64, t21749: f64, t136: f64, t11195: f64, t11204: f64, t14702: f64, t14766: f64, t18203: f64, t18219: f64, t18229: f64, t18494: f64, t18505: f64, t18512: f64, t21739: f64, t21741: f64, t21747: f64, t11147: f64, t20234: f64, t11145: f64, t123: f64, t11153: f64, t3240: f64, t21745: f64, t3242: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21750, t21751, t21753) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1046(t1113, t21749, t136, t11195, t11204, t14702, t14766, t18203, t18219, t18229, t18494, t18505, t18512, t21739, t21741, t21747);
        let t21758 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1047(t11147, t20234);
        let (t21759, t21760) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1048(t11145, t21758, t123);
        let t21762 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1049(t11153, t20234);
        let (t21763, t21764) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1050(t21762, t3240, t123);
        let (t21766, t21767) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1051(t21745, t3240, t123);
        let t21769 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1052(t20234, t3242);
    (t21750, t21751, t21753, t21758, t21759, t21760, t21762, t21763, t21764, t21766, t21767, t21769)
}
