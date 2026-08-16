//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1987/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1987(t2411: f64, t605: f64, t198: f64, t206: f64, t7086: f64, t25373: f64, t25392: f64, t25386: f64, t25372: f64, t2435: f64, t25352: f64, t11015: f64, t7018: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92790 = t2411 * t605;
    let t92819 = t198 * t206 * t7086;
    let t92837 = t25373 * t25392;
    let t92838 = t25386 * t92837;
    let t92843 = t25372 * t92837;
    let t92858 = t2435 * t25352;
    let t92861 = 0.30356481678079769392e-1_f64 * t7018 * t11015;
    (t92790, t92819, t92838, t92843, t92858, t92861)
}
