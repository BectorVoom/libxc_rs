//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2786/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2786<F: Float>(t39529: F, t40764: F, t40766: F, t40779: F, t40784: F, t40790: F, t40793: F, t40797: F, t40799: F, t58035: F, t58040: F, t58042: F, t58046: F, t58048: F, t58053: F, t58055: F, t58056: F, t58058: F, t58059: F) -> F {
    let t58970 = t58035 + t40764 + t40766 + t58040 + t58042 - t39529 + t58046 + t58048 - t40779 + t58053 + t40784 - t58055 - t58056 + t58058 + t40790 + t40793 + t58059 + t40797 + t40799;
    t58970
}
