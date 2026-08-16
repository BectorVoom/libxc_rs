//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 781/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk781(t11490: f64, t32606: f64, t23327: f64, t5631: f64, t32417: f64, t83: f64, t1825: f64, t452: f64, t7229: f64, t1307: f64, t5743: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32607 = t11490 * t32606;
    let t32610 = t23327 * t5631;
    let t32613 = t83 * t32417;
    let t32617 = t452 * t1825 * t7229;
    let t32620 = t1307 * t5743;
    let t32622 = t452 * t488 * t32620;
    (t32607, t32610, t32613, t32617, t32620, t32622)
}
