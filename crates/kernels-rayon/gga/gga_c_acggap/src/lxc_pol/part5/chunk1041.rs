//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1041/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1041(t17951: f64, t4264: f64, t1181: f64, t12991: f64, t3355: f64, t535: f64, t1164: f64, t4847: f64, t1446: f64, t3228: f64, t1352: f64, t3244: f64) -> (f64, f64, f64, f64, f64) {
    let t17952 = t17951 * t4264;
    let t17962 = t12991 * t1181 * t535 * t3355;
    let t17972 = t1164 * t4847;
    let t17984 = t3228 * t1446;
    let t18000 = t3244 * t1352;
    (t17952, t17962, t17972, t17984, t18000)
}
