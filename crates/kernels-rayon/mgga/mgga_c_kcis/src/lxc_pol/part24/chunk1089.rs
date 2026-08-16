//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1089/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1089(t236: f64, t28300: f64, t233: f64, t27836: f64, t8047: f64, t1020: f64, t3203: f64, t6276: f64, t7718: f64, t4555: f64, t6272: f64, t2842: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28301 = t236 * t28300;
    let t28302 = t233 * t28301;
    let t28904 = t27836 * t8047;
    let t28905 = t1020 * t28904;
    let t28907 = t3203 * t6276;
    let t28908 = t7718 * t28907;
    let t28909 = t1020 * t28908;
    let t28911 = t4555 * t6272;
    let t28912 = t7718 * t28911;
    let t28913 = t2842 * t28912;
    (t28302, t28904, t28905, t28907, t28908, t28909, t28911, t28912, t28913)
}
