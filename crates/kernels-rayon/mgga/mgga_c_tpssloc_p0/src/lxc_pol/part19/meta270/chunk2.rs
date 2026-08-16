//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1028/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1028(t1254: f64, t3637: f64, t3639: f64, t500: f64, t11405: f64, t11409: f64, t11426: f64, t11429: f64, t11472: f64, t11480: f64, t11482: f64, t11484: f64, t11631: f64, t11636: f64, t11940: f64, t1256: f64, t193: f64, t336: f64, t3633: f64, t3640: f64, t4700: f64) -> (f64, f64) {
    let t11944 = t3637 * t1254;
    let t11947 = 1.0_f64 / t3639 / t500;
    let t11955 = t11940 * t1256 * t193 * t336 + 2.0_f64 * t11944 * t11947 * t193 * t336 - 3.0_f64 * t1254 * t3633 * t3640 * t4700 - t11405 + t11409 - t11426 + t11429 - t11472 - t11480 - t11482 - t11484 + t11631 - t11636;
    (t11947, t11955)
}
