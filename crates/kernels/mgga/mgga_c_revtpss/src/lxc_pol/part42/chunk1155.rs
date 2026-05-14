//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1155/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1155<F: Float>(t6152: F, t945: F, t15170: F, t15189: F, t15312: F, t15322: F, t15324: F, t18944: F, t18961: F, t18964: F, t18967: F, t18970: F, t18973: F, t11134: F, t11366: F, t11422: F, t11423: F, t18948: F, t19002: F, t19004: F, t19007: F, t19009: F, t19014: F, t19017: F) -> (F, F, F) {
    let t19173 = t6152 * t945;
    let t19202 = 0.103295e1 * t18944 + 0.20839e0 * t18961 - 0.69463333333333333334e-1 * t18964 - 0.46308888888888888889e-1 * t18967 - 0.62517e0 * t18970 + 0.41678e0 * t18973 - t15312 + 0.4630888888888888889e-1 * t15170 - 0.45908888888888888888e0 * t15189 + t15322 + t15324;
    let t19224 = -0.516475e0 * t18948 - t11422 - t11423 + 0.23154444444444444445e-1 * t19002 - 0.13892666666666666667e0 * t19004 - 0.104195e0 * t19007 + 0.69463333333333333333e-1 * t19009 - 0.22954444444444444444e0 * t11134 - 0.11577222222222222222e0 * t11366 + 0.20839e0 * t19014 - 0.34731666666666666667e-1 * t19017;
    (t19173, t19202, t19224)
}
