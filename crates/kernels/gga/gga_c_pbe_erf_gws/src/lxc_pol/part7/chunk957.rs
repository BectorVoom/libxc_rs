//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 957/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk957<F: Float>(t401: F, t5253: F, t1764: F, t177: F, t191: F, t16613: F, t16675: F, t16684: F, t16699: F, t16701: F, t16708: F, t16718: F, t16720: F, t16726: F, t16741: F, t16747: F, t1856: F, t25: F, t5264: F, t606: F) -> F {
    let t17745 = t401 * t5253;
    let t17758 = t191 / t177 / t1764;
    let t17765 = F::cast_from(0.28793333333333333333e0_f64) * t16701 - F::cast_from(0.28793333333333333333e0_f64) * t16708 - F::cast_from(0.23994444444444444446e0_f64) * t16720 + F::cast_from(0.95977777777777777777e-1_f64) * t16726 - F::cast_from(0.88888888888888888888e-2_f64) * t25 * t1856 * t16684 - F::cast_from(0.17777777777777777778e-1_f64) * t25 * t5264 * t16718 + F::cast_from(0.17777777777777777778e-1_f64) * t17745 - F::new(0.24e0) * t25 * t606 * t16613 + F::cast_from(0.53333333333333333332e-1_f64) * t25 * t606 * t16699 + F::cast_from(0.79999999999999999998e-1_f64) * t25 * t1856 * t16675 - F::cast_from(0.69135802469135802468e-2_f64) * t25 * t17758 * t16741 - F::cast_from(0.66666666666666666667e-2_f64) * t25 * t606 * t16747;
    t17765
}
