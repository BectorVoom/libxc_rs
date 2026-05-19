//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1275/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1275<F: Float>(t101849: F, t101853: F, t27607: F, t28714: F, t28749: F, t28755: F, t28760: F, t28772: F, t29510: F, t7971: F, t8213: F, t98978: F, t98986: F, t98988: F, t99013: F, t99331: F) -> F {
    let t101862 = F::cast_from(0.34752604166666666667e-3_f64) * t27607 * t29510 + F::cast_from(0.69505208333333333334e-3_f64) * t99013 * t8213 - F::cast_from(0.82448622685185185187e-4_f64) * t101849 + F::cast_from(0.69505208333333333334e-3_f64) * t28714 * t28772 - t98978 - t98986 - t98988 - F::cast_from(0.18534722222222222222e-2_f64) * t101853 * t7971 - F::cast_from(0.61782407407407407408e-3_f64) * t99331 * t28749 - F::cast_from(0.61782407407407407408e-3_f64) * t99331 * t28755 - F::cast_from(0.12356481481481481482e-2_f64) * t99331 * t28760;
    t101862
}
