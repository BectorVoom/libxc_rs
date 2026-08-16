//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 723/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk723(t45: f64, t57: f64, t2219: f64, t1289: f64, t80: f64, t3431: f64, t581: f64, t741: f64, t83: f64, t745: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t3594 = 0.5848223622634646207e0_f64 * t2219;
    let t3595 = t80 * t1289;
    let t3601 = piecewise3(t151, 0.0_f64, -2.0_f64 / 9.0_f64 * t3595 * t581 + 2.0_f64 / 3.0_f64 * t741 * t3431);
    let t3602 = t83 * t1289;
    let t3608 = piecewise3(t155, 0.0_f64, -2.0_f64 / 9.0_f64 * t3602 * t581 - 2.0_f64 / 3.0_f64 * t745 * t3431);
    let t3610 = t3601 / 2.0_f64 + t3608 / 2.0_f64;
    (t3594, t3595, t3602, t3610)
}
