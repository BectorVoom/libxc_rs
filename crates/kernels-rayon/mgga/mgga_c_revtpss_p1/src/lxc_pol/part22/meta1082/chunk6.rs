//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3907/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3907(t10090: f64, t122: f64, t14144: f64, t2482: f64, t6861: f64, t72: f64, t9994: f64, t14145: f64, t4114: f64, t10014: f64, t22336: f64, t46496: f64, t46500: f64, t46505: f64, t48049: f64, t48058: f64, t48066: f64, t75014: f64, t75018: f64, t75021: f64, t75024: f64, t75026: f64) -> f64 {
    let t75035 = t2482 * t10090 * t6861 * t9994 * t72 * t122 * t14144;
    let t75039 = t2482 * t4114 * t6861 * t14145;
    let t75041 = t10014 * t22336;
    let t75044 = -0.39029762157531132076e-1_f64 * t48049 - 0.23131639038696784278e-2_f64 * t46496 + 0.21951497276451705328e-1_f64 * t75014 + 0.10975748638225852664e-1_f64 * t75018 + 0.13009920719177044025e-1_f64 * t75021 - 0.39029762157531132076e-1_f64 * t48058 - 0.39029762157531132074e-1_f64 * t75024 + 0.65049603595885220126e-3_f64 * t75026 + 0.11565819519348392139e-2_f64 * t46500 + 0.10975748638225852664e-1_f64 * t48066 - 0.11708928647259339622e0_f64 * t75035 + 0.11708928647259339622e0_f64 * t75039 - 0.19514881078765566038e-1_f64 * t75041 + 0.92526556154787137112e-2_f64 * t46505;
    t75044
}
