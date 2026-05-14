//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 677/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk677<F: Float>(t2049: F, t2666: F, t7300: F, t7305: F, t7308: F, t7313: F, t7318: F, t7321: F, t7323: F, t7325: F, t7328: F, t7331: F, t7334: F, t7338: F, t7402: F, t7407: F, t7411: F, t7414: F, t7416: F, t7418: F, t7420: F, t7422: F, t7425: F, t7427: F, t7432: F, t7434: F, t7438: F, t7441: F) -> (F, F, F) {
    let t7659 = t2666 * t2049;
    let t7675 = -0.625e-1 * t7300 - 0.20833333333333333333e-1 * t7305 - 0.625e-1 * t7308 - 0.44965277777777777777e-2 * t7313 + 0.1875e0 * t7318 + 0.101171875e-1 * t7321 + 0.625e-1 * t7323 + 0.101171875e-1 * t7325 - 0.53958333333333333333e-1 * t7328 - 0.16666666666666666667e0 * t7331 + 0.13489583333333333333e-1 * t7334 - 0.9375e-1 * t7338 + 0.9375e-1 * t7402;
    let t7689 = -0.9375e-1 * t7407 - 0.101171875e-1 * t7411 + 0.71944444444444444444e-1 * t7414 - 0.13489583333333333333e-1 * t7416 + 0.53958333333333333333e-1 * t7418 - 0.13489583333333333333e-1 * t7420 - 0.9375e-1 * t7422 + 0.25e0 * t7425 - 0.25e0 * t7427 - 0.20234375e-1 * t7432 + 0.625e-1 * t7434 + 0.101171875e-1 * t7438 + 0.13489583333333333333e-1 * t7441;
    (t7659, t7675, t7689)
}
