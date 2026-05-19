//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 702/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk702<F: Float>(t1490: F, t7931: F, t303: F, t1498: F, t553: F, t2237: F, t2239: F, t7895: F, t7898: F, t7901: F, t7906: F, t7908: F, t7911: F, t7916: F, t7922: F, t7926: F, t7929: F) -> (F, F, F, F, F) {
    let t7932 = t7931 * t1490;
    let t7933 = t303 * t7932;
    let t7935 = t553 * t1498;
    let t7936 = t303 * t7935;
    let t7938 = -F::cast_from(0.69505208333333333333e-3_f64) * t7895 * t2239 + F::cast_from(0.92754700520833333333e-4_f64) * t7898 * t7901 - t7906 - F::cast_from(0.23168402777777777778e-3_f64) * t7908 * t7911 + F::cast_from(0.69505208333333333333e-3_f64) * t2237 * t7916 + F::cast_from(0.69505208333333333333e-3_f64) * t2237 * t7901 + t7922 + F::cast_from(0.16581944444444444444e-2_f64) * t7926 + F::cast_from(0.24872916666666666666e-2_f64) * t7929 - F::cast_from(0.24872916666666666666e-2_f64) * t7933 + F::cast_from(0.16581944444444444444e-2_f64) * t7936;
    (t7932, t7933, t7935, t7936, t7938)
}
