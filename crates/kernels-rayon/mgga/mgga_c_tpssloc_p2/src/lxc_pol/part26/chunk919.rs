//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 919/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk919(t10789: f64, t10804: f64, t932: f64, t2884: f64, t922: f64, t302: f64, t2887: f64, t310: f64, t10743: f64, t2791: f64, t888: f64, t2794: f64) -> (f64, f64, f64, f64) {
    let t10805 = t10789 + t10804;
    let t10806 = t10805 * t932;
    let t10810 = 1.0_f64 / t2884 / t922;
    let t10811 = t302 * t10810;
    let t10813 = 1.0_f64 / t2887 / t310;
    let t10814 = t10743 * t10813;
    let t10817 = t888 * t2791;
    let t10819 = 6.0_f64 * t10817 * t2794;
    (t10806, t10811, t10814, t10819)
}
