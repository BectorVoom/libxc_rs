//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 857/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk857(t1307: f64, t6557: f64, t452: f64, t488: f64, t5710: f64, t6478: f64, t23339: f64, t6538: f64, t11810: f64, t23323: f64, t6534: f64, t32488: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34661 = t1307 * t6557;
    let t34663 = t452 * t488 * t34661;
    let t34667 = t452 * t5710 * t6478;
    let t34670 = t23339 * t6538;
    let t34671 = t11810 * t34670;
    let t34674 = t23323 * t6534;
    let t34677 = t32488 * t925;
    (t34661, t34663, t34667, t34670, t34671, t34674, t34677)
}
