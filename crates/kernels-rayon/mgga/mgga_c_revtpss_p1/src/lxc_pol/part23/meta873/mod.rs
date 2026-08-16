//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta873 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2774;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta873(t13845: f64, t13847: f64, t5675: f64, t73856: f64, t22107: f64, t9962: f64, t1399: f64, t22245: f64, t2661: f64, t3992: f64, t221: f64, t22287: f64, t3978: f64, t9921: f64, t22289: f64, t3989: f64, t1868: f64, t1883: f64, t46825: f64, t9793: f64, t47274: f64, t6849: f64, t9816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t74469, t74471, t74475, t74477) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2774(t13845, t13847, t5675, t73856, t22107, t9962, t1399, t22245, t2661, t3992, t221, t22287);
        let (t74479, t74481, t74483, t74485, t74489) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2775(t3978, t74477, t9921, t22289, t3989, t1868, t1883, t46825, t9793, t1399, t47274, t6849, t9816);
    (t74469, t74471, t74475, t74479, t74481, t74483, t74485, t74489)
}
