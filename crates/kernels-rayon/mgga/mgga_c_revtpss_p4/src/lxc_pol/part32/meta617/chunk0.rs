//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1957/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1957(t22295: f64, t26028: f64, t22299: f64, t22093: f64, t22098: f64, t2018: f64, t22129: f64, t807: f64, t22262: f64, t25986: f64, t2661: f64, t22182: f64, t94508: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t108543 = t26028 * t22295;
    let t108545 = t26028 * t22299;
    let t108547 = t26028 * t22093;
    let t108549 = t26028 * t22098;
    let t108554 = t807 * t2018 * t22129;
    let t108559 = t2661 * t25986 * t22262;
    let t108562 = t94508 * t22182;
    (t108543, t108545, t108547, t108549, t108554, t108559, t108562)
}
