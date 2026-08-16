//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 741/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk741(t15593: f64, t947: f64, t3108: f64, t4414: f64, t1537: f64, t4500: f64, t4436: f64, t7241: f64, t432: f64, t28: f64, t89: f64, t4418: f64, t7780: f64) -> (f64, f64, f64, f64, f64) {
    let t15594 = t15593 * t947;
    let t15596 = t4414 * t3108;
    let t15599 = t1537 * t4500;
    let t15601 = t7241 * t4436;
    let t15602 = t15601 * t432;
    let t15604 = t89 * t28 * t15602;
    let t15606 = t89 * t7780 * t4418;
    (t15594, t15596, t15599, t15604, t15606)
}
