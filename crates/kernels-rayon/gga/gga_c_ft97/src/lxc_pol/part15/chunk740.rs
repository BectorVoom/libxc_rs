//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 740/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk740(t17016: f64, t925: f64, t2210: f64, t167: f64, t20035: f64, t569: f64, t1060: f64, t4458: f64, t20660: f64, t9432: f64, t12664: f64, t4724: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20874 = t17016 * t925;
    let t20875 = t2210 * t20874;
    let t20880 = t569 * t167 * t20035;
    let t20884 = t569 * t1060 * t4458;
    let t20888 = t9432 * t167 * t20660;
    let t20893 = t12664 * t4724;
    (t20874, t20875, t20880, t20884, t20888, t20893)
}
