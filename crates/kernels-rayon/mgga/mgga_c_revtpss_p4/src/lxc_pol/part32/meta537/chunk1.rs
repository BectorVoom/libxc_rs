//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1848/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1848(t45972: f64, t7342: f64, t10309: f64, t26178: f64, t94973: f64, t530: f64, t7535: f64, t198: f64, t206: f64, t7427: f64, t25373: f64, t26550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95316 = t45972 * t7342;
    let t95319 = t10309 * t26178;
    let t95397 = 308.0_f64 / 27.0_f64 * t94973;
    let t95472 = t530 * t7535;
    let t95511 = t198 * t206 * t7427;
    let t95536 = t25373 * t26550;
    (t95316, t95319, t95397, t95472, t95511, t95536)
}
