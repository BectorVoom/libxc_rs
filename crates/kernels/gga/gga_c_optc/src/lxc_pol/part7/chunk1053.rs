//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1053/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1053<F: Float>(t23845: F, t23913: F, t23622: F, t23624: F, t23626: F, t23630: F, t23633: F, t23635: F, t23637: F, t23640: F, t23644: F, t23647: F, t23651: F, t23653: F, t23655: F, t23682: F) -> (F, F, F) {
    let t23914 = t23913 * t23845;
    let t23918 = -0.53675555555555555556e0 * t23622 + 0.40256666666666666668e0 * t23624 + 0.44729629629629629629e0 * t23626 - 0.89459259259259259259e0 * t23630 - 0.301925e0 * t23633 + 0.12524296296296296297e1 * t23635 - 0.16102666666666666667e1 * t23637 + 0.40256666666666666666e1 * t23640 + 0.181155e1 * t23644 + 0.198684e1 * t23647 + 0.49671e0 * t23651 - 0.485484375e1 * t23914 - 0.24154e1 * t23653 + 0.80513333333333333333e0 * t23655;
    let t23926 = 0.31310740740740740741e1 * t23682;
    (t23914, t23918, t23926)
}
