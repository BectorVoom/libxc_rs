//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 875/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk875(t13620: f64, t2087: f64, t4614: f64, t13631: f64, t825: f64, t826: f64, t2684: f64, t7354: f64, t45423: f64, t7427: f64, t7573: f64, t10915: f64, t22242: f64, t45316: f64) -> (f64, f64, f64, f64, f64) {
    let t45611 = 0.92023022289409799224e1_f64 * t2087 * t4614 * t13620;
    let t45613 = t825 * t826 * t13631;
    let t45614 = 0.25561950635947166451e0_f64 * t45613;
    let t45616 = t2684 * t7354 * t13631;
    let t45617 = 0.25561950635947166451e0_f64 * t45616;
    let t45627 = 0.62115540045351614476e2_f64 * t7427 * t7573 * t45423;
    let t45630 = 0.21450293971110256001e1_f64 * t22242 * t10915 * t45316;
    (t45611, t45614, t45617, t45627, t45630)
}
