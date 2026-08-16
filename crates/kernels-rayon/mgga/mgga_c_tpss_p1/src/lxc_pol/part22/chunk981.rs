//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 981/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk981(t45: f64, t57: f64, t3431: f64, t80: f64, t10353: f64, t1310: f64, t1985: f64, t1992: f64, t3595: f64, t581: f64, t741: f64, t83: f64, t1311: f64, t3602: f64, t745: f64, zeta_threshold: f64) -> (f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t10531 = t80 * t3431;
    let t10539 = piecewise3(t151, 0.0_f64, 8.0_f64 / 27.0_f64 * t1310 * t1985 - 4.0_f64 / 9.0_f64 * t10531 * t581 - 2.0_f64 / 9.0_f64 * t3595 * t1992 + 2.0_f64 / 3.0_f64 * t741 * t10353);
    let t10542 = t83 * t3431;
    let t10550 = piecewise3(t155, 0.0_f64, -8.0_f64 / 27.0_f64 * t1311 * t1985 - 4.0_f64 / 9.0_f64 * t10542 * t581 - 2.0_f64 / 9.0_f64 * t3602 * t1992 - 2.0_f64 / 3.0_f64 * t745 * t10353);
    (t10539, t10550)
}
