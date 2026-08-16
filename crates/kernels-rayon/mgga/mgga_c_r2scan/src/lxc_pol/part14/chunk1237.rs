//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1237/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1237(t11336: f64, t37327: f64, t39268: f64, t3275: f64, t3472: f64, t40609: f64, t10940: f64, t12203: f64, t41329: f64, t41332: f64, t41335: f64, t41339: f64, t41342: f64, t41346: f64, t41350: f64, t41786: f64, t41788: f64, t41790: f64, t41794: f64, t41797: f64, t41800: f64) -> (f64, f64, f64, f64) {
    let t41803 = 15.0_f64 / 8.0_f64 * t37327 * t11336 * t39268;
    let t41806 = 5.0_f64 / 8.0_f64 * t3275 * t3472 * t40609;
    let t41808 = 5.0_f64 / 16.0_f64 * t10940 * t12203;
    let t41809 = -t41329 + t41332 + t41335 + t41339 + t41342 + t41346 - t41350 + t41786 + t41788 + t41790 + t41794 + t41797 - t41800 + t41803 - t41806 - t41808;
    (t41803, t41806, t41808, t41809)
}
