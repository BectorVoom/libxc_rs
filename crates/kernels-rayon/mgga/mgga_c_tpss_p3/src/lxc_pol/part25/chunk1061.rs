//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1061/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1061(t2504: f64, t4854: f64, t849: f64, t11002: f64, t11050: f64, t11051: f64, t11071: f64, t14551: f64, t14553: f64, t14556: f64, t14559: f64, t14561: f64, t8665: f64) -> (f64, f64) {
    let t14563 = t2504 * t4854;
    let t14564 = t14563 * t849;
    let t14568 = -t8665 + 0.142419375e1_f64 * t14551 - 0.1898925e1_f64 * t14553 - 0.9494625e0_f64 * t14556 - 0.76790625e-1_f64 * t14559 + 0.3071625e0_f64 * t14561 + 0.15358125e0_f64 * t14564 - t11050 + 0.36514074074074074073e-1_f64 * t11051 + 0.13287407407407407407e0_f64 * t11002 - t11071;
    (t14564, t14568)
}
