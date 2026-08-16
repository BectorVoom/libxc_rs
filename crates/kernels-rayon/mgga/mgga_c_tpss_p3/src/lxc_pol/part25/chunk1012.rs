//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1012/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1012(t45: f64, t8006: f64, t4573: f64, t608: f64, t4579: f64, t80: f64, t13335: f64, t3431: f64, t3595: f64, t581: f64, t741: f64, t612: f64, t83: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t14003 = 0.5848223622634646207e0_f64 * t8006;
    let t14004 = t608 * t4573;
    let t14009 = t80 * t4579;
    let t14015 = piecewise3(t151, 0.0_f64, 8.0_f64 / 27.0_f64 * t14004 * t581 - 4.0_f64 / 9.0_f64 * t3595 * t3431 - 2.0_f64 / 9.0_f64 * t14009 * t581 + 2.0_f64 / 3.0_f64 * t741 * t13335);
    let t14016 = t612 * t4573;
    let t14021 = t83 * t4579;
    (t14003, t14015, t14016, t14021)
}
