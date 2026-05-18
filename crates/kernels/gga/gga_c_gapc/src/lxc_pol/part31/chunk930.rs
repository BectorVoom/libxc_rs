//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 930/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk930<F: Float>(t8927: F, t8930: F, t8933: F, t8935: F, t8938: F, t8941: F, t8943: F, t8945: F, t8952: F, t8961: F, t8963: F, t8966: F, t8969: F) -> F {
    let t10648 = -F::new(0.38647271295071362317e-7) * t8927 + F::new(0.14492726735651760868e-5) * t8930 + F::new(0.2471588561924985691e-3) * t8933 + F::new(0.74218967013888888897e-4) * t8935 + F::new(0.13900948042322754167e-3) * t8938 - F::new(0.74147656857749570729e-3) * t8941 + F::new(0.16682738775705804733e-3) * t8943 - F::new(0.1349435763888888889e-4) * t8945 - F::new(0.19679271556712962965e-5) * t8952 + F::new(0.86096813060619212971e-6) * t8961 + F::new(0.2471588561924985691e-3) * t8963 + F::new(0.28985453471303521736e-5) * t8966 + F::new(0.4048307291666666667e-4) * t8969;
    t10648
}
