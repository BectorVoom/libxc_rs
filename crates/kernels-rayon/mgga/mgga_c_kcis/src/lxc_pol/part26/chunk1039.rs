//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1039/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1039(t2157: f64, t26450: f64, t137: f64, t2425: f64, t86: f64, t2421: f64, t695: f64, t8939: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26451 = t26450 * t2157;
    let t26454 = t86 * t2425 * t137;
    let t26457 = t86 * t2421 * t137;
    let t26459 = t8939 * t695;
    let t26460 = t26459 * t2157;
    let t26462 = t695 * t68;
    (t26451, t26454, t26457, t26459, t26460, t26462)
}
