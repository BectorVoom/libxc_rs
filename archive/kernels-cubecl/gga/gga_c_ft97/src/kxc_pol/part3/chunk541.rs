//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 541/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk541<F: Float>(t1221: F, t1882: F, t2655: F, t2658: F, t2793: F, t4032: F, t4035: F, t4039: F, t4042: F, t4046: F, t4049: F, t4054: F, t4059: F, t4132: F, t4193: F, t4228: F) -> (F, F) {
    let t4283 = t1882 * t1221;
    let t4299 = -t4193 / F::cast_from(4.0_f64) + t4228 / F::cast_from(2.0_f64) + t2793 + t2655 / F::cast_from(9.0_f64) + t2658 / F::cast_from(3.0_f64) + t4032 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4035 + t4039 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4042 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4046 + t4049 / F::cast_from(3.0_f64) + t4054 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) * t4059 - t4132;
    (t4283, t4299)
}
