//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 901/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk901(t11114: f64, t11118: f64, t11399: f64, t11404: f64, t11409: f64, t11411: f64, t11445: f64, t11450: f64, t11453: f64, t11456: f64, t11461: f64, t11466: f64, t11468: f64, t11502: f64, t11507: f64, t11510: f64, t11513: f64, t11517: f64, t2938: f64, t2943: f64, t2963: f64, t2968: f64, t2971: f64, t2982: f64, t3007: f64, t3015: f64, t946: f64, t955: f64, t965: f64, t974: f64) -> f64 {
    let t11520 = t11114 - t11118 + 3.0_f64 * t11399 * t955 + 3.0_f64 * t2938 * t2963 + 0.96491876992155210402e2_f64 * t11404 * t2971 - 0.19298375398431042081e3_f64 * t11409 * t11411 + 1.0_f64 * t946 * t11445 + 0.2069040516770936012e4_f64 * t11450 * t11453 + 0.17544670867903938621e1_f64 * t11456 * t974 + 0.17544670867903938621e1_f64 * t2982 * t3007 + 0.51947577317044391276e2_f64 * t11461 * t3015 - 0.10389515463408878255e3_f64 * t11466 * t11468 + 0.5848223622634646207e0_f64 * t965 * t11502 + 0.10254018858216406658e4_f64 * t11507 * t11510 - 6.0_f64 * t2943 * t11513 + 0.96491876992155210402e2_f64 * t2968 * t11517;
    t11520
}
