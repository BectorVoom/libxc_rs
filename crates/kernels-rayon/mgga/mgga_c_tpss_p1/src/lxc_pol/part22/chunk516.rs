//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 516/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk516(t45: f64, t57: f64, t1985: f64, t1992: f64, t741: f64, t80: f64, t745: f64, t83: f64, zeta_threshold: f64) -> f64 {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t2125 = piecewise3(t151, 0.0_f64, -2.0_f64 / 9.0_f64 * t80 * t1985 + 2.0_f64 / 3.0_f64 * t741 * t1992);
    let t2131 = piecewise3(t155, 0.0_f64, -2.0_f64 / 9.0_f64 * t83 * t1985 - 2.0_f64 / 3.0_f64 * t745 * t1992);
    let t2133 = t2125 / 2.0_f64 + t2131 / 2.0_f64;
    t2133
}
