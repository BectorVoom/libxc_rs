//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2338/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2338(t28904: f64, t576: f64, t28868: f64, t580: f64, t100900: f64, t100942: f64, t1398: f64, t1404: f64, t1858: f64, t20149: f64, t20186: f64, t2023: f64, t2029: f64, t26510: f64, t28869: f64, t5364: f64, t6471: f64, t7020: f64, t7774: f64, t86565: f64, t86567: f64, t86571: f64, t96348: f64) -> f64 {
    let t100945 = t576 * t28904;
    let t100946 = t28868 * t580;
    let t100948 = t86565 + 2.0_f64 * t26510 * t1858 + t86567 + 2.0_f64 * t5364 * t7774 + t6471 * t7020 + t28869 * t1404 + t20149 * t2029 + t96348 + t1398 * (t100900 + t100942) + t100945 + t86571 + t100946 + t2023 * t20186;
    t100948
}
