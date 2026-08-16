//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 616/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk616(t1289: f64, t725: f64, t681: f64, t150: f64, t3589: f64, t190: f64, t1352: f64, t2208: f64, t2217: f64, t2245: f64, t2292: f64, t2302: f64, t2310: f64, t2333: f64, t2347: f64, t2351: f64, t3594: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3642 = t725 * t1289;
    let t3643 = t681 * t3642;
    let t3644 = 4.0_f64 * t3643;
    let t3645 = t150 * t3589;
    let t3646 = t3645 * t190;
    let t3647 = t1352 * t725;
    let t3648 = t2351 + t2310 - t2208 - t2217 - t3594 + t2347 + t3644 - t2292 + t2302 + t2245 + t2333 + t3646 + t3647;
    (t3642, t3643, t3644, t3645, t3646, t3647, t3648)
}
