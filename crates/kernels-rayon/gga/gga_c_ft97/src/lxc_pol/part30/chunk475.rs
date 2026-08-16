//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 475/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk475(t2680: f64, t7584: f64, t317: f64, t193: f64, t213: f64, t7464: f64) -> (f64, f64, f64, f64) {
    let t7585 = t2680 * t7584;
    let t7586 = t7585 * t317;
    let t7587 = t193 * t7586;
    let t7590 = t213 * t7464;
    (t7585, t7586, t7587, t7590)
}
