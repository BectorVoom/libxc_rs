//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 399/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk399<F: Float>(t110: F, t10: F, t107: F, t119: F, t142: F, t3020: F, t64: F, t903: F, t41: F, t120: F, t912: F, t919: F, t212: F, t9: F) -> (F, F, F, F) {
    let t111 = t110 < -0.66725e-1;
    let t3031 = piecewise3(t111, 0.0, 10.0 / 9.0 * t64 * t3020 * t10 - 20.0 / 27.0 * t64 * t903 * t142 + 40.0 / 81.0 * t64 * t107 * t119);
    let t3032 = t3031 * t41;
    let t3033 = t3032 * t120;
    let t3036 = t912 * t919;
    let t3042 = 1.0 / t9 / t212;
    (t3032, t3033, t3036, t3042)
}
