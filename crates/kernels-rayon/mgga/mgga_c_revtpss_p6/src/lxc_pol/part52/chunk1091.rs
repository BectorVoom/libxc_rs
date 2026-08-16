//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1091/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1091(t5: f64, t32584: f64, t32590: f64, t32599: f64, t33621: f64, t34169: f64, t34173: f64, t34177: f64, t34181: f64, t8620: f64, t8623: f64, t117: f64, t7935: f64, t8698: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t34187 = piecewise3(t8, 0.0_f64, -5.0_f64 / 72.0_f64 * t34169 * t8623 + 5.0_f64 / 12.0_f64 * t32584 * t34173 + 5.0_f64 / 18.0_f64 * t32590 * t34177 + t32599 - 5.0_f64 / 36.0_f64 * t8620 * t34181 - 5.0_f64 / 72.0_f64 * t8620 * t33621);
    let t34188 = t34187 * t117;
    let t34191 = t8698 * t7935;
    (t34187, t34188, t34191)
}
