//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 585/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk585(t24330: f64, t6832: f64, t6055: f64, t172: f64, t6818: f64, t6820: f64, t6815: f64, t6043: f64, t6824: f64, t6808: f64, t6809: f64, t1109: f64, t6022: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27569 = t24330 * t6832;
    let t27570 = t6055 * t27569;
    let t27574 = t6818 * t172;
    let t27575 = t27574 * t6820;
    let t27576 = t6815 * t27575;
    let t27579 = t6043 * t24330 * t6824;
    let t27582 = t6808 * t24330 * t6809;
    let t27584 = t6022 * t1109;
    (t27569, t27570, t27575, t27576, t27579, t27582, t27584)
}
