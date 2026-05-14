//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 389/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk389<F: Float>(t319: F, t4129: F, t840: F, t1221: F, t1882: F, t2655: F, t2658: F, t2793: F, t4032: F, t4035: F, t4039: F, t4042: F, t4046: F, t4049: F, t4054: F, t4059: F, t4132: F, t4193: F, t4228: F) -> (F, F, F) {
    let t4280 = t840 * t319 * t4129;
    let t4283 = t1882 * t1221;
    let t4299 = -t4193 / 4.0 + t4228 / 2.0 + t2793 + t2655 / 9.0 + t2658 / 3.0 + t4032 / 9.0 - 2.0 / 9.0 * t4035 + t4039 / 3.0 + 2.0 / 3.0 * t4042 + 2.0 / 3.0 * t4046 + t4049 / 3.0 + t4054 / 3.0 + 2.0 * t4059 - t4132;
    (t4280, t4283, t4299)
}
