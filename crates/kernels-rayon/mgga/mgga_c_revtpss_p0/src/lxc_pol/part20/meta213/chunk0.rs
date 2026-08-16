//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 994/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk994(t45: f64, t10439: f64, t10440: f64, t2401: f64, t750: f64, t200: f64, t2375: f64, t606: f64, t10326: f64, t10356: f64, t2258: f64, t78: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t10442 = 24.0_f64 * t10439 * t10440;
    let t10443 = t2401 * t750;
    let t10444 = 3.0_f64 * t10443;
    let t10446 = 1.0_f64 / t200 / t45;
    let t10449 = t2375 * t606;
    let t10455 = piecewise3(t151, 0.0_f64, -8.0_f64 / 27.0_f64 * t10446 * t10356 + 4.0_f64 / 3.0_f64 * t10449 * t2258 + 4.0_f64 / 3.0_f64 * t78 * t10326);
    (t10442, t10444, t10446, t10449, t10455)
}
