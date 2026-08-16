//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 964/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk964(t18866: f64, t18868: f64, t18870: f64, t18872: f64, t18874: f64, t18947: f64, t18949: f64, t18970: f64, t18973: f64, t18976: f64, t18980: f64, t20377: f64, t405: f64) -> f64 {
    let t20380 = -0.3109e-1_f64 * t20377 * t405 - t18866 - t18868 - t18870 + t18872 - t18874 - t18947 - t18949 + t18970 + t18973 + t18976 - t18980;
    t20380
}
