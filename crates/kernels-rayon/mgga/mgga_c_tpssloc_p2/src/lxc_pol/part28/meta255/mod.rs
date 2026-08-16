//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta255 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1105;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1106;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1107;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1108;
use chunk4::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1109;
use chunk5::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta255(t6635: f64, t6644: f64, t2047: f64, t814: f64, t829: f64, t235: f64, t7084: f64, t2051: f64, t226: f64, t6641: f64, t6650: f64, t6654: f64, t808: f64, t812: f64, t858: f64, t2054: f64, t259: f64, t2597: f64, t2713: f64, t6557: f64, t6569: f64, t6574: f64, t7067: f64, t7069: f64, t7072: f64, t7085: f64, t7087: f64, t7092: f64, t855: f64, t866: f64, t870: f64, t2056: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7095, t7097, t7101) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1105(t6635, t6644, t2047, t814);
        let (t7102, t7104, t7106) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1106(t7101, t829, t235, t7084, t2051, t226, t6641, t6650, t6654, t7095, t7097, t808, t812);
        let t7107 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1107(t7106, t858);
        let t7109 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1108(t2054, t259, t2597, t2713, t6557, t6569, t6574, t7067, t7069, t7072, t7085, t7087, t7092, t7107, t855, t866);
        let t7110 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1109(t7109, t870);
        let t7114 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1110(t2056, t2752);
    (t7095, t7097, t7101, t7102, t7104, t7106, t7107, t7109, t7110, t7114)
}
