//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 995/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk995(t3267: f64, t5415: f64, t10081: f64, t5383: f64, t124: f64, t13671: f64, t762: f64, t12817: f64, t12822: f64, t12828: f64, t13698: f64, t3273: f64, t3275: f64) -> (f64, f64, f64, f64, f64) {
    let t13725 = t3267 * t5415;
    let t13727 = t10081 * t5383;
    let t13730 = t124 * t13671;
    let t13731 = t762 * t13730;
    let t13736 = t12822 * t12828 * t12817;
    let t13741 = t3273 * t13698 * t3275;
    (t13725, t13727, t13731, t13736, t13741)
}
