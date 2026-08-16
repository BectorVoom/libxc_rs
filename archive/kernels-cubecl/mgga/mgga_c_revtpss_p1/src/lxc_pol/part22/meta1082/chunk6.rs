//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3907/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3907<F: Float>(t10090: F, t122: F, t14144: F, t2482: F, t6861: F, t72: F, t9994: F, t14145: F, t4114: F, t10014: F, t22336: F, t46496: F, t46500: F, t46505: F, t48049: F, t48058: F, t48066: F, t75014: F, t75018: F, t75021: F, t75024: F, t75026: F) -> F {
    let t75035 = t2482 * t10090 * t6861 * t9994 * t72 * t122 * t14144;
    let t75039 = t2482 * t4114 * t6861 * t14145;
    let t75041 = t10014 * t22336;
    let t75044 = -F::cast_from(0.39029762157531132076e-1_f64) * t48049 - F::cast_from(0.23131639038696784278e-2_f64) * t46496 + F::cast_from(0.21951497276451705328e-1_f64) * t75014 + F::cast_from(0.10975748638225852664e-1_f64) * t75018 + F::cast_from(0.13009920719177044025e-1_f64) * t75021 - F::cast_from(0.39029762157531132076e-1_f64) * t48058 - F::cast_from(0.39029762157531132074e-1_f64) * t75024 + F::cast_from(0.65049603595885220126e-3_f64) * t75026 + F::cast_from(0.11565819519348392139e-2_f64) * t46500 + F::cast_from(0.10975748638225852664e-1_f64) * t48066 - F::cast_from(0.11708928647259339622e0_f64) * t75035 + F::cast_from(0.11708928647259339622e0_f64) * t75039 - F::cast_from(0.19514881078765566038e-1_f64) * t75041 + F::cast_from(0.92526556154787137112e-2_f64) * t46505;
    t75044
}
