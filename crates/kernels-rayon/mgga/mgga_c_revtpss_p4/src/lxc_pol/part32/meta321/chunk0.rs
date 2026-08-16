//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1238/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1238(t2434: f64, t371: f64, t373: f64, t367: f64, t1065: f64, t675: f64, t247: f64, t906: f64, t1063: f64, t1062: f64, t3223: f64, t1052: f64, t3147: f64) -> (f64, f64, f64, f64, f64) {
    let t11970 = t371 * t2434 * t373;
    let t11972 = 0.63517063878621832551e-4_f64 * t367 * t11970;
    let t11986 = t675 * t1065;
    let t11988 = t247 * t11986 * t906;
    let t11989 = t1063 * t11988;
    let t11994 = t3223 * t1062;
    let t11997 = t1052 * t3147;
    (t11972, t11986, t11989, t11994, t11997)
}
