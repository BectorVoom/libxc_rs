//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 925/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk925<F: Float>(t8927: F, t8930: F, t8933: F, t8935: F, t8938: F, t8941: F, t8943: F, t8945: F, t8952: F, t8961: F, t8963: F, t8966: F, t8969: F) -> F {
    let t10648 = -F::cast_from(0.38647271295071362317e-7_f64) * t8927 + F::cast_from(0.14492726735651760868e-5_f64) * t8930 + F::cast_from(0.2471588561924985691e-3_f64) * t8933 + F::cast_from(0.74218967013888888897e-4_f64) * t8935 + F::cast_from(0.13900948042322754167e-3_f64) * t8938 - F::cast_from(0.74147656857749570729e-3_f64) * t8941 + F::cast_from(0.16682738775705804733e-3_f64) * t8943 - F::cast_from(0.1349435763888888889e-4_f64) * t8945 - F::cast_from(0.19679271556712962965e-5_f64) * t8952 + F::cast_from(0.86096813060619212971e-6_f64) * t8961 + F::cast_from(0.2471588561924985691e-3_f64) * t8963 + F::cast_from(0.28985453471303521736e-5_f64) * t8966 + F::cast_from(0.4048307291666666667e-4_f64) * t8969;
    t10648
}
