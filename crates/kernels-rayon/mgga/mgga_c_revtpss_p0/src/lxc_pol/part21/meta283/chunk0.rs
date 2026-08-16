//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1517/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1517(t2367: f64, t625: f64, t654: f64, t2340: f64, t665: f64, t2339: f64, t2366: f64, t2269: f64, t98: f64, t99: f64, t2350: f64, t658: f64, tau0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10206 = t625 * t2367;
    let t10207 = t654 * t654;
    let t10208 = 1.0_f64 / t10207;
    let t10209 = t2340 * t665;
    let t10210 = t10208 * t10209;
    let t10213 = t2339 * t665;
    let t10214 = t10213 * t2366;
    let t10217 = tau0 * t2269;
    let t10226 = t99 * t98;
    let t10227 = 1.0_f64 / t10226;
    let t10228 = t2350 * t658;
    (t10206, t10207, t10208, t10209, t10210, t10214, t10217, t10227, t10228)
}
