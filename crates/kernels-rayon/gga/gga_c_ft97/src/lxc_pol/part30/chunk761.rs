//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 761/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk761(t242: f64, t33608: f64, t684: f64, t724: f64, t7560: f64, t10157: f64, t265: f64, t33302: f64, t7440: f64, t766: f64, t2574: f64, t762: f64) -> (f64, f64, f64, f64, f64) {
    let t33609 = t242 * t33608;
    let t33613 = t724 * t7560 * t684;
    let t33617 = t10157 * t265 * t33302;
    let t33620 = t7440 * t766;
    let t33622 = t2574 * t762 * t33620;
    (t33609, t33613, t33617, t33620, t33622)
}
