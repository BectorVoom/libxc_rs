//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 864/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk864(t5986: f64, t5834: f64, t5963: f64, t5966: f64, t5968: f64, t5970: f64, t5972: f64, t5975: f64, t5976: f64, t5978: f64, t5982: f64, t5985: f64) -> f64 {
    let t7849 = 80.0_f64 * t5986;
    let t7850 = t5963 - t5966 + 0.43374325201206959368e-1_f64 * t5968 - 0.64212977516902094772e0_f64 * t5970 - 0.2602459512072417562e0_f64 * t5972 - t5975 + 16.0_f64 * t5976 - 0.2258170631111111111e-2_f64 * t5978 + t5834 - 40.0_f64 * t5982 + t5985 - t7849;
    t7850
}
