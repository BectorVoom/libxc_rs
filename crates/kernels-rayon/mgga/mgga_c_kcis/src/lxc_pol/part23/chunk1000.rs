//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1000/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1000(t2128: f64, t4500: f64, t12930: f64, t12933: f64, t17307: f64, t17313: f64, t17315: f64, t17319: f64, t17322: f64, t17328: f64, t4475: f64, t4480: f64, t6222: f64, t6225: f64, t6256: f64) -> (f64, f64) {
    let t18355 = t2128 * t4500;
    let t18364 = -t12930 * t2128 + 4.0_f64 * t12933 * t6225 + 2.0_f64 * t18355 * t4480 - 2.0_f64 * t4475 * t6256 - t4500 * t6222 - t17307 - t17313 + t17315 + t17319 + t17322 - t17328;
    (t18355, t18364)
}
