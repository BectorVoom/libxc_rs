//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1070/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1070(t14701: f64, t912: f64, t10980: f64, t11002: f64, t11276: f64, t11277: f64, t14459: f64, t14492: f64, t14495: f64, t14505: f64, t14507: f64, t14517: f64, t14521: f64, t14525: f64, t14528: f64, t14532: f64, t14535: f64, t8616: f64, t8927: f64) -> (f64, f64) {
    let t14703 = 0.23392894490538584828e1_f64 * t912 * t14701;
    let t14719 = -t8927 - 0.76103703703703703703e-2_f64 * t8616 - 0.1522074074074074074e-1_f64 * t10980 + 0.761037037037037037e-2_f64 * t11002 - t11276 + t11277 + 0.3805185185185185185e-2_f64 * t14495 - 0.19025925925925925925e-1_f64 * t14517 + 0.68493333333333333331e-1_f64 * t14459 - 0.2283111111111111111e-1_f64 * t14521 - 0.11415555555555555555e-1_f64 * t14505 - 0.10274e0_f64 * t14525 + 0.68493333333333333332e-1_f64 * t14528 + 0.57077777777777777777e-2_f64 * t14507 - 0.11415555555555555555e-1_f64 * t14532 + 0.34246666666666666666e-1_f64 * t14535 - 0.17123333333333333333e-1_f64 * t14492;
    (t14703, t14719)
}
