//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 726/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk726(t2689: f64, t810: f64, t775: f64, t854: f64, t236: f64, t807: f64, t21: f64, t65: f64) -> (f64, f64, f64, f64, f64) {
    let t2691 = 0.76220476654346199061e-4_f64 * t2689 * t810;
    let t2693 = t854 * t775;
    let t2694 = t236 * t2693;
    let t2695 = t807 * t2694;
    let t2698 = 1.0_f64 / t65 / t21;
    (t2691, t2693, t2694, t2695, t2698)
}
