//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1233/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1233(t11438: f64, t26331: f64, t5549: f64, t1030: f64, t33307: f64, t4979: f64, t11513: f64, t1749: f64, t5285: f64, t1743: f64, t34123: f64, t11449: f64, t11519: f64, t1845: f64, t190: f64) -> (f64, f64, f64, f64, f64) {
    let t34638 = t11438 * t26331 * t5549;
    let t34641 = t1030 * t33307 * t4979;
    let t34644 = t5285 * t11513 * t1749;
    let t34647 = t1743 * t34123 * t4979;
    let t34651 = t1845 * t190 * t11449 * t11519;
    (t34638, t34641, t34644, t34647, t34651)
}
