//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 608/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk608<F: Float>(t4226: F, t845: F, t91: F, t4032: F, t4049: F, t2656: F, t2659: F, t2823: F, t4035: F, t4039: F, t4042: F, t4046: F, t4054: F, t4059: F, t4132: F, t4193: F) -> (F, F) {
    let t4228 = t91 * t845 * t4226;
    let t4230 = t4032 / F::cast_from(27.0_f64);
    let t4235 = t4049 / F::cast_from(9.0_f64);
    let t4239 = -t4193 / F::cast_from(12.0_f64) + t4228 / F::cast_from(6.0_f64) + t2823 + t2656 + t2659 + t4230 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t4035 + t4039 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4042 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t4046 + t4235 + t4054 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4059 - t4132 / F::cast_from(3.0_f64);
    (t4228, t4239)
}
