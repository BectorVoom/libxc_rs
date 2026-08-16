//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 967/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk967(t30: f64, t33: f64, t1289: f64, t1985: f64, t7737: f64, t2009: f64, t3431: f64, t581: f64, t1992: f64, t3446: f64, t555: f64, t7622: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t10340 = t7737 * t1289 * t1985;
    let t10343 = t2009 * t3431;
    let t10344 = t10343 * t581;
    let t10347 = t3446 * t1992;
    let t10350 = 2.0_f64 * t555;
    let t10351 = 6.0_f64 * t7622;
    let t10353 = piecewise5(t31, 0.0_f64, t34, 0.0_f64, t10350 - t10351);
    (t10340, t10344, t10347, t10353)
}
