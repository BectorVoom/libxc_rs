//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 491/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk491<F: Float>(t3746: F, t683: F, t3051: F, t2401: F, t2402: F, t3738: F, t3741: F, t3744: F) -> (F, F, F) {
    let t3747 = t683 * t3746;
    let t3748 = t3051 * t3747;
    let t3750 = t2401 + t2402 / F::cast_from(9.0_f64) + t3738 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3741 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3744 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3748;
    (t3747, t3748, t3750)
}
