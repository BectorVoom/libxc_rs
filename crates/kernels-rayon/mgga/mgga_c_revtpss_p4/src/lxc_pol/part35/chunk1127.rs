//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1127/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1127(t10985: f64, t26576: f64, t2062: f64, t2769: f64, t786: f64, t2070: f64, t41154: f64, t25876: f64, t26304: f64, t25894: f64, t2097: f64, t22: f64, t25937: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95930 = 0.46263278077393568556e-2_f64 * t26576 * t10985;
    let t95936 = t786 * t2062 * t2769;
    let t95964 = t2070 * t41154;
    let t96186 = t25876 * t26304;
    let t96187 = t25894 * t96186;
    let t96204 = t25937 * t2097 * t22;
    (t95930, t95936, t95964, t96186, t96187, t96204)
}
