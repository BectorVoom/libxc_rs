//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 733/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk733(t1912: f64, t3045: f64, t5285: f64, t511: f64, t670: f64, t22: f64, t1900: f64, t1743: f64, t5703: f64, t3103: f64, t577: f64, t3109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8650 = t5285 * t3045 * t1912;
    let t8652 = t670 * t511;
    let t8654 = 1.0_f64 / t22 / t8652;
    let t8655 = t1900 * t8654;
    let t8657 = t1743 * t8655 * t5703;
    let t8659 = t577 * t3103;
    let t8660 = t8659 * t3109;
    (t8650, t8652, t8654, t8655, t8657, t8660)
}
