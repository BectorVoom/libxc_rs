//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 686/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk686(t574: f64, t616: f64, t6615: f64, t1901: f64, t26925: f64, t26929: f64, t26932: f64, t26936: f64, t26940: f64, t26943: f64, t26947: f64, t26952: f64, t26957: f64, t26961: f64, t26965: f64, t26969: f64, t446: f64) -> f64 {
    let t26973 = t574 * t616 * t6615;
    let t26976 = -2.0_f64 / 3.0_f64 * t1901 * t26925 - 2.0_f64 / 3.0_f64 * t1901 * t26929 + t1901 * t26932 / 9.0_f64 - t1901 * t26936 / 9.0_f64 + t446 * t26940 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t26943 + t446 * t26947 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t26952 + t446 * t26957 / 3.0_f64 - t446 * t26961 / 3.0_f64 - t446 * t26965 / 3.0_f64 - t446 * t26969 / 3.0_f64 - t446 * t26973 / 3.0_f64;
    t26976
}
