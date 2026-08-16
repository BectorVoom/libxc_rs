//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1860/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1860(t7493: f64, t9292: f64, t136: f64, t137: f64, t2097: f64, t94386: f64, t94391: f64, t9646: f64, t9648: f64, t25875: f64, t96186: f64, t26230: f64, t94633: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96218 = 0.17073386770573548589e-1_f64 * t9292 * t7493;
    let t96220 = t2097 * t136 * t137;
    let t96221 = t96220 * t94386;
    let t96222 = t94391 * t96221;
    let t96230 = 0.19637199382202157274e-3_f64 * t9646 * t2097 * t9648;
    let t96236 = t25875 * t96186;
    let t96245 = t26230 * t94633;
    (t96218, t96220, t96221, t96222, t96230, t96236, t96245)
}
