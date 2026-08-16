//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 630/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk630(t1053: f64, t1102: f64, t3457: f64, t3267: f64, t3273: f64, t3280: f64, t3351: f64, t3355: f64, t3432: f64, t3442: f64, t3445: f64, t3451: f64, t3455: f64) -> f64 {
    let t3459 = t1102 * t1053 * t3457;
    let t3461 = -t3432 + t3442 - t3445 - t3451 - 0.36021158228745895953e-3_f64 * t3455 + 0.15243824895787514157e-3_f64 * t3459 - t3267 - t3273 + t3280 - t3351 + t3355;
    t3461
}
