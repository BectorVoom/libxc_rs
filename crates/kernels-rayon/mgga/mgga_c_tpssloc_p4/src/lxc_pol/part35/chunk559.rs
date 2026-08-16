//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 559/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk559(t1222: f64, t1731: f64, t1744: f64, t1229: f64, t3247: f64, t3242: f64, t3584: f64, t1653: f64, t248: f64, t3521: f64) -> (f64, f64, f64, f64, f64) {
    let t4957 = t1731 * t1222;
    let t4959 = t1744 * t1222;
    let t4972 = t1229 * t3247;
    let t4987 = t3584 * t3242;
    let t4993 = t248 * t3521 * t1653;
    (t4957, t4959, t4972, t4987, t4993)
}
