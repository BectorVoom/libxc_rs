//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 786/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk786(t3365: f64, t5432: f64, t1260: f64, t1640: f64, t220: f64, t3370: f64, t339: f64, t4511: f64, t523: f64, t5381: f64, t5408: f64, t5413: f64, t5427: f64) -> (f64, f64) {
    let t5433 = t3365 * t5432;
    let t5448 = -t1260 * t339 * t5408 - t1260 * t339 * t5413 - 2.0_f64 * t1640 * t339 * t4511 + t220 * t523 * t5427 + 2.0_f64 * t3370 * t339 * t5381;
    (t5433, t5448)
}
