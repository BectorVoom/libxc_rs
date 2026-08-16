//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1104/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1104(t1081: f64, t12135: f64, t11875: f64, t11942: f64, t11873: f64, t11880: f64, t11885: f64, t11890: f64, t11896: f64, t11899: f64, t11904: f64, t11908: f64, t11938: f64, t11952: f64, t9221: f64, t9223: f64, t9226: f64, t9228: f64, t9477: f64) -> (f64, f64) {
    let t12136 = t12135 * t1081;
    let t12145 = 0.2283111111111111111e-1_f64 * t11875;
    let t12146 = 0.11415555555555555555e-1_f64 * t11942;
    let t12155 = -t9477 + 0.1522074074074074074e-1_f64 * t9221 + 0.38051851851851851851e-2_f64 * t9223 - 0.11415555555555555555e-1_f64 * t9226 - 0.57077777777777777777e-2_f64 * t9228 + 0.76103703703703703702e-2_f64 * t11938 + 0.76103703703703703701e-2_f64 * t11873 - t12145 - t12146 + 0.19025925925925925925e-1_f64 * t11880 - 0.68493333333333333331e-1_f64 * t11885 - 0.2283111111111111111e-1_f64 * t11890 - 0.11415555555555555555e-1_f64 * t11896 + 0.10274e0_f64 * t11899 + 0.68493333333333333332e-1_f64 * t11904 + 0.34246666666666666666e-1_f64 * t11908 + 0.17123333333333333333e-1_f64 * t11952;
    (t12136, t12155)
}
