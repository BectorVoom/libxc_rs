//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1127/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1127(t139212: f64, t139224: f64, t27147: f64, t32899: f64, t139213: f64, t27158: f64, t631: f64, t95262: f64, t147647: f64, t23667: f64, t5899: f64, t34808: f64, t379: f64) -> (f64, f64, f64, f64) {
    let t148270 = t139212 * t139224 * t32899 * t27147;
    let t148275 = t95262 * t631 * t139213 * t32899 * t27158;
    let t148278 = t5899 * t23667 * t147647;
    let t148280 = t34808 * t379;
    (t148270, t148275, t148278, t148280)
}
