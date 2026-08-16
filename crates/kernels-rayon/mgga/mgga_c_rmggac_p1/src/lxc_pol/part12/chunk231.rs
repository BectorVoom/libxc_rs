//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 231/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk231(t410: f64, t417: f64, t971: f64, t431: f64, t176: f64, t409: f64, t416: f64) -> (f64, f64, f64, f64) {
    let t973 = t410 * t971 * t417;
    let t975 = 0.5848223622634646207e0_f64 * t431 * t973;
    let t976 = t409 * t176;
    let t977 = 1.0_f64 / t976;
    let t978 = t416 * t416;
    (t973, t975, t977, t978)
}
