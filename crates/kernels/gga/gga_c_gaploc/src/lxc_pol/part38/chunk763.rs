//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 763/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk763<F: Float>(t2562: F, t35558: F, t883: F, t943: F, t13542: F, t2549: F, t11613: F, t1897: F, t7675: F, t2508: F, t33760: F, t9014: F, t35719: F, t954: F, t44707: F, t688: F, t779: F) -> (F, F, F, F, F, F) {
    let t45028 = t943 * t2562 * t883 * t35558;
    let t45029 = 0.32043859292259267849e-3 * t45028;
    let t45030 = t2549 * t13542;
    let t45031 = 0.32043859292259267849e-3 * t45030;
    let t45034 = 0.92286314761706691403e-1 * t1897 * t11613 * t7675;
    let t45037 = 0.18457262952341338281e0 * t2508 * t9014 * t33760;
    let t45044 = 0.15381052460284448567e-1 * t2508 * t954 * t35719;
    let t45048 = 0.76905262301422242837e-2 * t2508 * t779 * t44707 * t688;
    (t45029, t45031, t45034, t45037, t45044, t45048)
}
