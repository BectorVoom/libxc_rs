//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1514/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1514(t221: f64, t23177: f64, t2484: f64, t2485: f64, t1469: f64, t4401: f64, t61303: f64, t14613: f64, t18539: f64, t18544: f64, t4311: f64, t23214: f64, t750: f64) -> (f64, f64, f64, f64, f64) {
    let t76887 = t2484 * t2485 * t221 * t23177;
    let t76892 = t4401 * t61303 * t1469;
    let t76947 = t14613 * t18539;
    let t76949 = t4311 * t18544;
    let t76951 = t23214 * t750;
    (t76887, t76892, t76947, t76949, t76951)
}
