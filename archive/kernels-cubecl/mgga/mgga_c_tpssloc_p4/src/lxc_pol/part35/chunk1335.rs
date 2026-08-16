//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1335/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1335<F: Float>(t8074: F, t94909: F, t24826: F, t29745: F, t24574: F, t29705: F, t477: F, t6238: F, t29777: F, t29678: F, t7359: F, t29759: F) -> (F, F, F, F, F, F, F) {
    let t103867 = t94909 * t8074;
    let t103877 = t24826 * t29745;
    let t103879 = t24574 * t29705;
    let t103881 = t477 * t6238;
    let t103927 = t24574 * t29777;
    let t103939 = t29678 * t7359;
    let t103943 = t24574 * t29759;
    (t103867, t103877, t103879, t103881, t103927, t103939, t103943)
}
