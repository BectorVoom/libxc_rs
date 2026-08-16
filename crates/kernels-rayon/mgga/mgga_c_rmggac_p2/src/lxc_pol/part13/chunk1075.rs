//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1075/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1075(t40479: f64, t40505: f64, t35742: f64, t35744: f64, t35752: f64, t35766: f64, t40481: f64, t40489: f64, t40491: f64, t40493: f64, t40495: f64, t40502: f64, t40507: f64, t40509: f64, t40511: f64, t40513: f64, t40516: f64) -> f64 {
    let t43433 = 0.39726959900411316772e-4_f64 * t40479;
    let t43440 = 0.39726959900411316772e-4_f64 * t40505;
    let t43446 = 0.60975299583150056624e-3_f64 * t35742 + 0.60975299583150056624e-3_f64 * t35744 + 0.47896966807455234256e0_f64 * t35752 + 0.15965655602485078085e0_f64 * t35766 + t43433 + 0.1702583995731913576e-4_f64 * t40481 + 0.2727466165424534173e-1_f64 * t40489 - 0.40911992481368012596e-1_f64 * t40491 + 0.5454932330849068346e-1_f64 * t40493 - 0.85129199786595678799e-5_f64 * t40495 + 0.1702583995731913576e-4_f64 * t40502 - t43440 - 0.1702583995731913576e-4_f64 * t40507 - 0.85129199786595678799e-5_f64 * t40509 + 0.5107751987195740728e-4_f64 * t40511 + 0.2553875993597870364e-4_f64 * t40513 - 0.95793933614910468511e0_f64 * t40516;
    t43446
}
