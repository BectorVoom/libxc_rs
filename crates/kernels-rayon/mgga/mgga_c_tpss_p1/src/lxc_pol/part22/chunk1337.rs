//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1337/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1337(t18464: f64, t4484: f64, t13015: f64, t5728: f64, t1646: f64, t60749: f64, t19506: f64, t5570: f64, t13032: f64, t1705: f64, t935: f64, t1232: f64, t4516: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65643 = t18464 * t4484;
    let t65645 = t5728 * t13015;
    let t65647 = t60749 * t1646;
    let t65667 = t19506 * t5570;
    let t65685 = t1705 * t13032 * t935;
    let t65691 = t4516 * t1232 * t520;
    (t65643, t65645, t65647, t65667, t65685, t65691)
}
