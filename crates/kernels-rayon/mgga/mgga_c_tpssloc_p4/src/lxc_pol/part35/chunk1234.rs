//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1234/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1234(t1246: f64, t29719: f64, t29708: f64, t3612: f64, t2147: f64, t6238: f64, t462: f64, t1409: f64, t1734: f64, t7376: f64, t24851: f64, t1653: f64, t27460: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29720 = t29719 * t1246;
    let t29723 = t29708 * t3612;
    let t29726 = t2147 * t6238;
    let t29727 = t462 * t29726;
    let t29734 = t1409 * t1734;
    let t29735 = t29734 * t7376;
    let t29736 = t24851 * t29735;
    let t29740 = t27460 * t1653;
    (t29720, t29723, t29726, t29727, t29735, t29736, t29740)
}
