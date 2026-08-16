//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 639/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk639(t255: f64, t28298: f64, t10051: f64, t1449: f64, t3864: f64, t6917: f64, t9787: f64, t1091: f64, t24599: f64, t2606: f64, t24793: f64, t3870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28299 = t28298 * t255;
    let t28300 = t10051 * t1449;
    let t28301 = t28300 * t3864;
    let t28302 = t28299 * t28301;
    let t28305 = t9787 * t6917;
    let t28308 = t24599 * t1091;
    let t28309 = t2606 * t28308;
    let t28312 = t24793 * t3870;
    (t28299, t28300, t28301, t28302, t28305, t28308, t28309, t28312)
}
