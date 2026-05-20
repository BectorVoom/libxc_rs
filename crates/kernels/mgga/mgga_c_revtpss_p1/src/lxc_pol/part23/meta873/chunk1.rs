//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2775/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2775<F: Float>(t3978: F, t74477: F, t9921: F, t22289: F, t3989: F, t1868: F, t1883: F, t46825: F, t9793: F, t1399: F, t47274: F, t6849: F, t9816: F) -> (F, F, F, F, F) {
    let t74479 = t3978 * t9921 * t74477;
    let t74481 = t3989 * t22289;
    let t74483 = t1883 * t1868;
    let t74485 = t9793 * t46825 * t74483;
    let t74489 = t9816 * t47274 * t6849 * t1399;
    (t74479, t74481, t74483, t74485, t74489)
}
