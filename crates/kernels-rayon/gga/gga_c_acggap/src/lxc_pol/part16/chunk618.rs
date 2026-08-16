//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 618/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk618(t1181: f64, t1182: f64, t5862: f64, t1838: f64, t435: f64, t1165: f64, t1188: f64, t407: f64, t1772: f64, t301: f64, t1089: f64, t368: f64) -> (f64, f64, f64, f64, f64) {
    let t5864 = t1181 * t5862 * t1182;
    let t5867 = t435 * t1838;
    let t5869 = t1165 * t5867 * t1188;
    let t5873 = t1165 * t5862 * t407;
    let t5876 = t1772 * t301;
    let t5878 = t1089 * t368 * t5876;
    (t5864, t5869, t5873, t5876, t5878)
}
