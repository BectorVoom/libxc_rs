//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 901/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk901<F: Float>(t2858: F, t2873: F, t3649: F, t485: F, t395: F, t3652: F, t9779: F, t9781: F, t9784: F, t9789: F, t9794: F, t9796: F, t9799: F, t9802: F) -> (F, F, F, F) {
    let t10046 = t2858 * t2873;
    let t10049 = t485 * t3649;
    let t10050 = t10049 * t395;
    let t10051 = F::cast_from(0.97434166666666666667e0_f64) * t10050;
    let t10052 = t485 * t3652;
    let t10053 = t10052 * t395;
    let t10054 = F::cast_from(0.48717083333333333333e0_f64) * t10053;
    let t10063 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9779 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9781 - t9784 / F::cast_from(9.0_f64) + t9789 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t9794 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t9796 - t9799 / F::cast_from(9.0_f64) + t9802 / F::cast_from(3.0_f64);
    (t10046, t10051, t10054, t10063)
}
