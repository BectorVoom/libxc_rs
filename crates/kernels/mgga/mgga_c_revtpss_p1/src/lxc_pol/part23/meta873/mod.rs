//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta873 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2774;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2775;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta873<F: Float>(t13845: F, t13847: F, t5675: F, t73856: F, t22107: F, t9962: F, t1399: F, t22245: F, t2661: F, t3992: F, t221: F, t22287: F, t3978: F, t9921: F, t22289: F, t3989: F, t1868: F, t1883: F, t46825: F, t9793: F, t47274: F, t6849: F, t9816: F) -> (F, F, F, F, F, F, F, F) {
        let (t74469, t74471, t74475, t74477) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2774::<F>(t13845, t13847, t5675, t73856, t22107, t9962, t1399, t22245, t2661, t3992, t221, t22287);
        let (t74479, t74481, t74483, t74485, t74489) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2775::<F>(t3978, t74477, t9921, t22289, t3989, t1868, t1883, t46825, t9793, t1399, t47274, t6849, t9816);
    (t74469, t74471, t74475, t74479, t74481, t74483, t74485, t74489)
}
