//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1427/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1427<F: Float>(t3957: F, t6884: F, t124: F, t21969: F, t800: F, t6850: F, t9744: F, t125: F, t6861: F, t3936: F, t9835: F, t1414: F, t828: F) -> (F, F, F, F, F, F) {
    let t22038 = t3957 * t6884;
    let t22040 = t124 * t21969;
    let t22041 = t800 * t22040;
    let t22044 = t9744 * t6850;
    let t22046 = t125 * t6861;
    let t22048 = t3936 * t22046 * t9835;
    let t22052 = t1414 * t828 * t21969;
    (t22038, t22041, t22044, t22046, t22048, t22052)
}
