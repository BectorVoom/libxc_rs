//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 903/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk903(t10491: f64, t871: f64, t10695: f64, t311: f64, t309: f64, t10051: f64, t1160: f64, t265: f64, t42109: f64, t2486: f64, t2568: f64, t676: f64, t754: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44528 = t10491 * t871;
    let t44600 = 1.0_f64 / t10695 / t311;
    let t44601 = t309 * t44600;
    let t51340 = t1160 * t10051;
    let t51669 = t42109 * t265;
    let t51687 = t2486 * t2568;
    let t51853 = t676 * t754;
    (t44528, t44600, t44601, t51340, t51669, t51687, t51853)
}
