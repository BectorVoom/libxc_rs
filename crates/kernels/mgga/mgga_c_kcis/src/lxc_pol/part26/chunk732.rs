//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 732/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk732<F: Float>(t303: F, t8176: F, t2012: F, t553: F, t2237: F, t2239: F, t7898: F, t7906: F, t7908: F, t7922: F, t8144: F, t8148: F, t8151: F, t8155: F, t8159: F, t8166: F, t8169: F, t8172: F) -> (F, F, F, F) {
    let t8177 = t303 * t8176;
    let t8179 = t553 * t2012;
    let t8180 = t303 * t8179;
    let t8182 = -F::cast_from(0.69505208333333333333e-3_f64) * t8144 * t2239 + F::cast_from(0.92754700520833333333e-4_f64) * t7898 * t8148 + F::cast_from(0.18534722222222222222e-2_f64) * t8151 * t2239 - t7906 - F::cast_from(0.23168402777777777778e-3_f64) * t7908 * t8155 + F::cast_from(0.69505208333333333333e-3_f64) * t2237 * t8159 + F::cast_from(0.69505208333333333333e-3_f64) * t2237 * t8148 + t7922 + F::cast_from(0.16581944444444444444e-2_f64) * t8166 + F::cast_from(0.24872916666666666666e-2_f64) * t8169 - F::cast_from(0.24872916666666666666e-2_f64) * t8172 - F::cast_from(0.66327777777777777776e-2_f64) * t8177 + F::cast_from(0.16581944444444444444e-2_f64) * t8180;
    (t8177, t8179, t8180, t8182)
}
