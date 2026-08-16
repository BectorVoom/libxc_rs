//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 880/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk880<F: Float>(t2562: F, t35558: F, t883: F, t943: F, t13542: F, t2549: F, t11613: F, t1897: F, t7675: F, t2508: F, t33760: F, t9014: F) -> (F, F, F, F) {
    let t45028 = t943 * t2562 * t883 * t35558;
    let t45029 = F::cast_from(0.32043859292259267849e-3_f64) * t45028;
    let t45030 = t2549 * t13542;
    let t45031 = F::cast_from(0.32043859292259267849e-3_f64) * t45030;
    let t45034 = F::cast_from(0.92286314761706691403e-1_f64) * t1897 * t11613 * t7675;
    let t45037 = F::cast_from(0.18457262952341338281e0_f64) * t2508 * t9014 * t33760;
    (t45029, t45031, t45034, t45037)
}
