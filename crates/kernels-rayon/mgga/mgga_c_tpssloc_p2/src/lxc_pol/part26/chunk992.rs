//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 992/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk992(t11632: f64, t11637: f64, t491: f64, t1246: f64, t1222: f64, t3567: f64, t1203: f64, t3540: f64, t2393: f64, t374: f64, t486: f64, t485: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11638 = t11632 + t11637;
    let t11639 = t491 * t11638;
    let t11640 = t11639 * t1246;
    let t11642 = t3567 * t1222;
    let t11644 = t1203 * t3540;
    let t11647 = t374 * t2393 * t486;
    let t11649 = t485 * t11647 / 10368.0_f64;
    (t11638, t11639, t11640, t11642, t11644, t11647, t11649)
}
