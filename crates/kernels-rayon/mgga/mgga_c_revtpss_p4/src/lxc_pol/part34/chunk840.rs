//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 840/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk840(t12987: f64, t480: f64, t1224: f64, t3362: f64, t12268: f64, t3698: f64, t3367: f64, t404: f64, t12256: f64, t11239: f64, t460: f64, t1242: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12988 = t12987 * t480;
    let t13006 = t1224 * t3362;
    let t13020 = t3698 * t12268;
    let t13026 = 1.0_f64 / t404 / t3367;
    let t13027 = t13026 * t12256;
    let t13036 = t460 * t11239;
    let t13037 = t1242 * t1242;
    (t12988, t13006, t13020, t13027, t13036, t13037)
}
