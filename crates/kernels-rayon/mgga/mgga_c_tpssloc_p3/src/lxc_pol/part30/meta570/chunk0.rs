//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1940/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1940(t28550: f64, t28592: f64, t349: f64, t1945: f64, t5872: f64, t3201: f64, t1615: f64, t7593: f64, t1060: f64, t25523: f64, t7610: f64, t1539: f64, t25516: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28593 = t28550 + t28592;
    let t28594 = t349 * t28593;
    let t28596 = t1945 * t5872;
    let t28597 = t28596 * t3201;
    let t28601 = t7593 * t1615;
    let t28602 = t28601 * t1060;
    let t28605 = t25523 * t7610;
    let t28609 = t25516 * t1539;
    (t28593, t28594, t28596, t28597, t28601, t28602, t28605, t28609)
}
