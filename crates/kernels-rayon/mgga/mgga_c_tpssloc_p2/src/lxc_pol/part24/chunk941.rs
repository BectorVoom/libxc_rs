//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 941/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk941(t10619: f64, t324: f64, t300: f64, t2897: f64, t961: f64, t2940: f64, t2948: f64, t2928: f64, t941: f64, t2931: f64, t323: f64, t10524: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10620 = t10619 * t324;
    let t10622 = 0.19751673498613801407e-1_f64 * t300 * t10620;
    let t10623 = t300 * t2897;
    let t10625 = 0.17544670867903938621e1_f64 * t10623 * t961;
    let t10627 = 0.17544670867903938621e1_f64 * t2940 * t2948;
    let t10629 = 1.0_f64 / t2928 / t941;
    let t10632 = 1.0_f64 / t2931 / t323;
    let t10633 = t10629 * t10524 * t10632;
    (t10620, t10622, t10625, t10627, t10629, t10632, t10633)
}
