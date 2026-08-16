//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 611/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk611(t174: f64, t176: f64, t2641: f64, t6281: f64, t6284: f64, t44: f64, t6280: f64, t1926: f64, t447: f64, t1650: f64, t2011: f64, t4171: f64, t4170: f64, sigma2: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t175 = t174 <= zeta_threshold;
    let t6288 = piecewise3(t175, 0.0_f64, 4.0_f64 / 9.0_f64 * t2641 * t6281 + 4.0_f64 / 3.0_f64 * t176 * t6284);
    let t6290 = (t6280 + t6288) * t44;
    let t6887 = 1.0_f64 / t1926;
    let t6888 = sigma2 * t6887;
    let t6895 = piecewise3(t175, 0.0_f64, t6284);
    let t6896 = t447 * t6895;
    let t6903 = t1650 * t2011;
    let t6904 = t4171 * t6903;
    let t6905 = t4170 * t6904;
    (t6290, t6887, t6888, t6895, t6896, t6904, t6905)
}
