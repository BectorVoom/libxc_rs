//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2781/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2781<F: Float>(t40135: F, t760: F, t10565: F, t606: F, t706: F, t717: F, t10587: F, t2496: F, t39875: F, t39894: F, t9371: F, t39960: F, t39963: F) -> (F, F, F, F, F, F, F) {
    let t40137 = F::cast_from(0.6233709278045326953e3_f64) * t760 * t40135;
    let t40139 = t706 * t10565 * t606;
    let t40150 = t717 * t10565;
    let t40156 = t10587 * t2496;
    let t40165 = t39894 * t39875 * t9371;
    let t40167 = F::cast_from(0.12304822629859687989e5_f64) * t760 * t40165;
    let t40169 = t39960 * t39875 * t39963;
    (t40137, t40139, t40150, t40156, t40165, t40167, t40169)
}
