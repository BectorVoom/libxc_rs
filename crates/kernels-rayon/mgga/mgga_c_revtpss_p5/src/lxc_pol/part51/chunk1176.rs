//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1176/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1176(t119711: f64, t125961: f64, t125984: f64, t126017: f64, t126030: f64, t18875: f64, t1940: f64, t2403: f64, t25440: f64, t25445: f64, t27363: f64, t27375: f64, t27384: f64, t31876: f64, t4343: f64, t4433: f64, t4537: f64, t4541: f64, t7091: f64, t7782: f64, t8494: f64) -> f64 {
    let t127180 = -6.0_f64 * t119711 * t1940 * t27384 + 4.0_f64 * t125961 * t1940 * t25445 - 6.0_f64 * t125984 * t2403 * t7091 + 4.0_f64 * t126017 * t1940 * t25445 - 6.0_f64 * t126030 * t2403 * t7091 + 6.0_f64 * t18875 * t2403 * t31876 - 2.0_f64 * t1940 * t25440 * t7782 - 2.0_f64 * t1940 * t27363 * t7091 + 2.0_f64 * t1940 * t31876 * t4537 + 6.0_f64 * t2403 * t27375 * t31876 - 3.0_f64 * t2403 * t4343 * t8494 - 6.0_f64 * t4433 * t4541 * t8494;
    t127180
}
