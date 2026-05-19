//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1095/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1095<F: Float>(t6539: F, t7709: F, t5329: F, t1768: F, t27924: F, t303: F, t2173: F, t2175: F, t26781: F, t26837: F, t28928: F, t28932: F, t28936: F, t28939: F, t28945: F, t28948: F, t28952: F, t7703: F) -> (F, F, F, F, F) {
    let t28957 = t7709 * t6539;
    let t28958 = t5329 * t28957;
    let t28961 = t27924 * t1768;
    let t28962 = t303 * t28961;
    let t28964 = -F::cast_from(0.69505208333333333333e-3_f64) * t28932 * t2175 + F::cast_from(0.22109259259259259258e-2_f64) * t28936 + F::cast_from(0.46336805555555555556e-3_f64) * t7703 * t28939 + F::cast_from(0.46336805555555555556e-3_f64) * t7703 * t28928 + F::cast_from(0.16581944444444444444e-2_f64) * t28945 + F::cast_from(0.69505208333333333333e-3_f64) * t2173 * t28948 - F::cast_from(0.13901041666666666667e-2_f64) * t2173 * t28952 - t26837 - F::cast_from(0.185671721767578125e-4_f64) * t26781 * t28952 + F::cast_from(0.69505208333333333333e-3_f64) * t2173 * t28958 - F::cast_from(0.49745833333333333332e-2_f64) * t28962;
    (t28957, t28958, t28961, t28962, t28964)
}
