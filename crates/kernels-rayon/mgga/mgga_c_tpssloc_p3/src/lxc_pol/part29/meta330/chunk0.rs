//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1388/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1388(t1203: f64, t3540: f64, t2393: f64, t374: f64, t486: f64, t485: f64, t248: f64, t3516: f64, t3570: f64, t3515: f64, t3576: f64, t3604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11644 = t1203 * t3540;
    let t11647 = t374 * t2393 * t486;
    let t11649 = t485 * t11647 / 10368.0_f64;
    let t11651 = t248 * t3570 * t3516;
    let t11652 = t3515 * t11651;
    let t11665 = t3604 * t3576;
    (t11644, t11647, t11649, t11651, t11652, t11665)
}
