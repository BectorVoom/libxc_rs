//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1020/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1020(t2153: f64, t9268: f64, t7669: f64, t906: f64, t209: f64, t2403: f64, t2404: f64, t706: f64, t7589: f64, t2387: f64, t73: f64, t9251: f64) -> (f64, f64, f64, f64, f64) {
    let t26558 = t9268 * t2153;
    let t26561 = t7669 * t906;
    let t26571 = t209 * t2403 * t706 * t2404;
    let t26572 = t7589 * t26571;
    let t26576 = t209 * t73 * t9251 * t2387;
    (t26558, t26561, t26571, t26572, t26576)
}
