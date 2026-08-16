//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1060/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1060(t36766: f64, t8443: f64, t4601: f64, t8884: f64, t2191: f64, t8582: f64, t2868: f64, t7855: f64, t2057: f64, t26370: f64, t9000: f64, t9128: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41964 = t36766 * t8443;
    let t41969 = t4601 * t8884;
    let t41971 = t2191 * t8582;
    let t41973 = t2868 * t7855;
    let t41975 = t26370 * t2057;
    let t41977 = t9128 * t9000;
    (t41964, t41969, t41971, t41973, t41975, t41977)
}
