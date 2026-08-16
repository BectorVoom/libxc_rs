//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2577/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2577(t10008: f64, t545: f64, t689: f64, t869: f64, t4093: f64, t9292: f64, t10065: f64, t10073: f64, t1432: f64, t1433: f64, t39497: f64, t1385: f64) -> (f64, f64, f64, f64, f64) {
    let t47387 = t689 * t869 * t545 * t10008;
    let t47389 = t9292 * t4093;
    let t47391 = t10073 * t10065;
    let t47395 = 0.10118827226026589797e0_f64 * t1432 * t1433 * t39497;
    let t47396 = t1385 * t10008;
    (t47387, t47389, t47391, t47395, t47396)
}
