//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 321/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk321(t319: f64, t4129: f64, t840: f64, t1221: f64, t1882: f64, t2655: f64, t2658: f64, t2793: f64, t4032: f64, t4035: f64, t4039: f64, t4042: f64, t4046: f64, t4049: f64, t4054: f64, t4059: f64, t4132: f64, t4193: f64, t4228: f64) -> (f64, f64, f64) {
    let t4280 = t840 * t319 * t4129;
    let t4283 = t1882 * t1221;
    let t4299 = -t4193 / 4.0_f64 + t4228 / 2.0_f64 + t2793 + t2655 / 9.0_f64 + t2658 / 3.0_f64 + t4032 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t4035 + t4039 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t4042 + 2.0_f64 / 3.0_f64 * t4046 + t4049 / 3.0_f64 + t4054 / 3.0_f64 + 2.0_f64 * t4059 - t4132;
    (t4280, t4283, t4299)
}
