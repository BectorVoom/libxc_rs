//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1231/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1231(t1165: f64, t13299: f64, t13573: f64, t1532: f64, t17139: f64, t17314: f64, t17316: f64, t17318: f64, t17327: f64, t22538: f64, t22540: f64, t22544: f64, t22546: f64, t22550: f64, t301: f64, t3462: f64, t4257: f64, t525: f64, t6263: f64) -> f64 {
    let t22552 = -0.10289764348336736873e-1_f64 * t17314 + 0.34299214494455789578e-1_f64 * t17139 * t13299 * t525 * t4257 - 0.42874018118069736972e-3_f64 * t17316 + 0.24009450146119052704e-1_f64 * t17318 - 0.68598428988911579156e-2_f64 * t3462 * t1165 * t1532 * t6263 * t301 + 0.11337795902333997111e-1_f64 * t22538 - 0.40015750243531754508e-2_f64 * t22540 + 0.34299214494455789578e-2_f64 * t17327 - 0.25724410870841842183e-2_f64 * t13573 - 0.40015750243531754508e-1_f64 * t22544 + 0.80031500487063509015e-2_f64 * t22546 + 0.85748036236139473944e-3_f64 * t22550;
    t22552
}
