//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2781/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2781(t40135: f64, t760: f64, t10565: f64, t606: f64, t706: f64, t717: f64, t10587: f64, t2496: f64, t39875: f64, t39894: f64, t9371: f64, t39960: f64, t39963: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40137 = 0.6233709278045326953e3_f64 * t760 * t40135;
    let t40139 = t706 * t10565 * t606;
    let t40150 = t717 * t10565;
    let t40156 = t10587 * t2496;
    let t40165 = t39894 * t39875 * t9371;
    let t40167 = 0.12304822629859687989e5_f64 * t760 * t40165;
    let t40169 = t39960 * t39875 * t39963;
    (t40137, t40139, t40150, t40156, t40165, t40167, t40169)
}
