//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 799/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk799(t2582: f64, t7937: f64, t6063: f64, t7605: f64, t2155: f64, t2207: f64, t2208: f64, t2837: f64, t2559: f64, t7494: f64, t2526: f64, t277: f64) -> (f64, f64, f64, f64, f64) {
    let t7939 = 0.23115257973478049502e0_f64 * t2582 * t7937;
    let t7949 = t6063 * t7605;
    let t7951 = 0.19514881078765566037e-1_f64 * t2155 * t7949;
    let t7961 = t2207 * t2837 * t2208;
    let t7968 = 0.12805040077930161442e0_f64 * t7494 * t2559;
    let t7977 = t277 * t2526;
    (t7939, t7951, t7961, t7968, t7977)
}
