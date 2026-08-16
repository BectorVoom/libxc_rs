//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1034/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1034(t11002: f64, t11024: f64, t11028: f64, t11033: f64, t11037: f64, t11080: f64, t11083: f64, t11086: f64, t11089: f64, t11091: f64, t11094: f64, t11096: f64) -> (f64, f64) {
    let t11188 = 0.13418888888888888889e0_f64 * t11002;
    let t11205 = -0.20128333333333333333e0_f64 * t11024 - 0.181155e1_f64 * t11028 + 0.12077e1_f64 * t11033 + 0.60385e0_f64 * t11037 + 0.16504875e0_f64 * t11080 + 0.19419375e1_f64 * t11083 - 0.412621875e-1_f64 * t11086 - 0.258925e1_f64 * t11089 - 0.1294625e1_f64 * t11091 + 0.16504875e0_f64 * t11094 + 0.82524375e-1_f64 * t11096;
    (t11188, t11205)
}
