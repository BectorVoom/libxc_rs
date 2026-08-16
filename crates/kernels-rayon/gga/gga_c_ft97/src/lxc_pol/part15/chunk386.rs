//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 386/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk386(t668: f64, t870: f64, t170: f64, t2248: f64, t328: f64, t70: f64, t703: f64) -> (f64, f64, f64) {
    let t2882 = t870 * t668;
    let t2912 = 5.0_f64 / 18.0_f64 * t170 * t2248 * t328;
    let t2917 = t70 * t703;
    (t2882, t2912, t2917)
}
