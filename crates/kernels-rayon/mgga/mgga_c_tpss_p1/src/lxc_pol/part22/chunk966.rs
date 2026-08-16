//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 966/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk966(t1286: f64, t1980: f64, t1317: f64, t1982: f64, t3486: f64, t619: f64, t2049: f64, t1306: f64, t1985: f64, t1993: f64, t3462: f64, t582: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10292 = t1286 * t1980;
    let t10303 = t1317 * t1982;
    let t10306 = t3486 * t619;
    let t10309 = t1317 * t2049;
    let t10314 = t1985 * t1306;
    let t10317 = t1993 * t1306;
    let t10320 = t582 * t3462;
    (t10292, t10303, t10306, t10309, t10314, t10317, t10320)
}
