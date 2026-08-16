//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 809/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk809(t174: f64, t13014: f64, t1650: f64, t167: f64, t2641: f64, t160: f64, t176: f64, t2642: f64, t2645: f64, t4518: f64, t4521: f64, t740: f64, t833: f64, zeta_threshold: f64) -> f64 {
    let t175 = t174 <= zeta_threshold;
    let t13077 = t13014 * t1650;
    let t13080 = t2641 * t167;
    let t13091 = piecewise3(t175, 0.0_f64, -8.0_f64 / 27.0_f64 * t13077 * t2642 - 16.0_f64 / 9.0_f64 * t13080 * t740 * t833 + 4.0_f64 / 9.0_f64 * t4518 * t2645 - 8.0_f64 / 3.0_f64 * t176 * t740 + 8.0_f64 * t4521 * t160);
    t13091
}
