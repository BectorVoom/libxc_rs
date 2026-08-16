//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 670/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk670(t1526: f64, t1527: f64, t15562: f64, t15584: f64, t19950: f64, t19957: f64, t19961: f64, t19965: f64, t3088: f64, t342: f64, t343: f64, t4415: f64, t4422: f64, t4501: f64, t7704: f64) -> f64 {
    let t19969 = t4415 + t4501 + t7704 - t15562 / 18.0_f64 - t15584 / 6.0_f64 - t1526 * t3088 * t19950 / 9.0_f64 - t1526 * t1527 * t4422 / 6.0_f64 + t1526 * t1527 * t19957 / 6.0_f64 - t1526 * t1527 * t19961 / 12.0_f64 - t342 * t343 * t19965 / 4.0_f64;
    t19969
}
