//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 536/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk536<F: Float>(t44: F, t51: F, t3016: F, t471: F, t97: F, t2491: F, t1361: F, t2999: F, t3002: F, t48: F, t1368: F, t3007: F, t3010: F, t53: F, zeta_threshold: F) -> (F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t3018 = t97 * t471 * t3016;
    let t3019 = 3.0 * t3018;
    let t3020 = 0.11696447245269292414e1 * t2491;
    let t3026 = piecewise3(t45, 0.0, 4.0 / 9.0 * t1361 * t2999 + 4.0 / 3.0 * t48 * t3002);
    let t3032 = piecewise3(t52, 0.0, 4.0 / 9.0 * t1368 * t3007 + 4.0 / 3.0 * t53 * t3010);
    let t3033 = t3026 + t3032;
    (t3019, t3020, t3033)
}
