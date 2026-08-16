//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1285/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1285(t31386: f64, t6579: f64, t23012: f64, t8538: f64, t31339: f64, t81591: f64, t2047: f64, t213: f64, t225: f64, t31351: f64, t794: f64, t6562: f64, t6572: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114752 = t6579 * t31386;
    let t114759 = t23012 * t8538;
    let t114760 = 0.63969658155208805863e-1_f64 * t114759;
    let t114762 = t81591 * t31339;
    let t114770 = t213 * t2047 * t225;
    let t114785 = t31351 * t225;
    let t114790 = t794 * t2047;
    let t114792 = t6562 * t114790 * t6572;
    (t114752, t114760, t114762, t114770, t114785, t114790, t114792)
}
