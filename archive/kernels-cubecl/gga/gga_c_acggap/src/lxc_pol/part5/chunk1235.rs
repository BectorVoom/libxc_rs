//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1235/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1235<F: Float>(t16288: F, t16292: F, t16294: F, t16300: F, t16304: F, t21737: F, t21740: F, t21743: F, t21745: F, t21747: F, t21751: F, t21755: F, t21759: F, t21762: F) -> F {
    let t22605 = -t21737 / F::cast_from(2.0_f64) + t21740 / F::cast_from(3.0_f64) + t21743 / F::cast_from(6.0_f64) - t21745 / F::cast_from(3.0_f64) + t21747 / F::cast_from(6.0_f64) + t21751 / F::cast_from(6.0_f64) - t21755 / F::cast_from(12.0_f64) - t21759 / F::cast_from(12.0_f64) - F::cast_from(6.0_f64) * t21762 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t16288 - t16292 / F::cast_from(3.0_f64) - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t16294 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t16300 - t16304 / F::cast_from(12.0_f64);
    t22605
}
