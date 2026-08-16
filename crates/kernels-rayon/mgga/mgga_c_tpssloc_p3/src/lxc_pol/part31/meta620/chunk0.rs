//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1871/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1871(t22704: f64, t22705: f64, t28167: f64, t26331: f64, t26421: f64, t26446: f64, t5187: f64, t1992: f64, t22897: f64, t3792: f64, t57607: f64, t19745: f64, t81027: f64) -> (f64, f64, f64, f64) {
    let t96989 = t22704 * t22705 * t28167;
    let t96993 = t26331 * t26446 * t26421 * t5187;
    let t96997 = t1992 * t22897 * t57607 * t3792;
    let t97002 = t1992 * t81027 * t19745;
    (t96989, t96993, t96997, t97002)
}
