//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1305/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1305(t1235: f64, t5722: f64, t46: f64, t6515: f64, t6524: f64, t6456: f64, t3206: f64, t8446: f64, t926: f64, t2380: f64, t6475: f64, t8459: f64) -> (f64, f64, f64, f64, f64) {
    let t22919 = t1235 * t5722;
    let t22920 = t22919 * t46;
    let t22921 = t6515 * t22920;
    let t22924 = t6524 * t22920;
    let t22927 = t6456 * t22920;
    let t22933 = t3206 * t926 * t8446;
    let t22936 = t2380 * t6475 * t8459;
    (t22921, t22924, t22927, t22933, t22936)
}
