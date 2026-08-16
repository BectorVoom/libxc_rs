//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1291/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1291(t2592: f64, t8854: f64, t2969: f64, t7817: f64, t7329: f64, t8862: f64, t1960: f64, t977: f64, t1382: f64, t2497: f64, t2902: f64, t16710: f64, t1961: f64, t3459: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33980 = t2592 * t8854;
    let t33982 = t2969 * t7817;
    let t33988 = 4.0_f64 * t8862 * t7329;
    let t33991 = 2.0_f64 * t1960 * t8854 * t977;
    let t33997 = 4.0_f64 * t1382 * t2902 * t2497;
    let t34003 = 24.0_f64 * t16710 * t3459 * t1961;
    (t33980, t33982, t33988, t33991, t33997, t34003)
}
