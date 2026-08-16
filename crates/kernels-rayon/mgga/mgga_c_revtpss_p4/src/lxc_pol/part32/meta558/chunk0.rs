//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1877/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1877(t1873: f64, t94519: f64, t94520: f64, t94527: f64, t94537: f64, t94540: f64, t26004: f64, t5690: f64, t13951: f64, t2018: f64, t807: f64, t94565: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t98260 = t94519 * t1873;
    let t98263 = 35.0_f64 / 108.0_f64 * t94520;
    let t98264 = 0.1219527626469539185e-2_f64 * t94527;
    let t98267 = 0.10164000561857065645e-4_f64 * t94537;
    let t98268 = 0.72286371995927450868e-4_f64 * t94540;
    let t98269 = t26004 * t5690;
    let t98281 = t807 * t2018 * t13951;
    let t98283 = 0.18071592998981862717e-4_f64 * t94565;
    (t98260, t98263, t98264, t98267, t98268, t98269, t98281, t98283)
}
