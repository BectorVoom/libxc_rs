//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 528/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk528(t515: f64, t518: f64, t215: f64, t2559: f64, t535: f64, t1314: f64, t782: f64, t2566: f64, t795: f64, t154: f64, t557: f64, t205: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3704 = 1.0_f64 / t515;
    let t3711 = 1.0_f64 / t518;
    let t3725 = 0.64814814814814814813e-2_f64 * t2559 * t535 * t215;
    let t3726 = t782 * t1314;
    let t3731 = 0.26388888888888888888e-2_f64 * t2566 * t535 * t795;
    let t3732 = t154 * t557;
    let t3733 = t205 * t3732;
    (t3704, t3711, t3725, t3726, t3731, t3732, t3733)
}
