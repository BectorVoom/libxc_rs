//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1021/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1021<F: Float>(t40219: F, t41915: F, t41919: F, t41922: F, t41927: F, t41930: F, t41933: F, t41941: F, t41945: F, t41947: F, t41949: F, t41951: F, t47955: F, t47963: F, t47965: F, t47968: F, t47976: F, t47978: F, t47980: F, t47984: F) -> F {
    let t50871 = F::cast_from(0.92023022289409799224e1_f64) * t47955 + t41915 + t41919 + t41922 + F::cast_from(0.76685851907841499353e0_f64) * t40219 - t41927 - t41930 + t41933 + t41941 + t41945 - t47963 + F::cast_from(0.29792074959875355558e-1_f64) * t47965 + F::cast_from(0.29792074959875355558e-1_f64) * t47968 - F::cast_from(0.89376224879626066674e-1_f64) * t41947 - F::cast_from(0.89376224879626066674e-1_f64) * t41949 - F::cast_from(0.89376224879626066674e-1_f64) * t41951 + F::cast_from(0.29792074959875355558e-1_f64) * t47976 + F::cast_from(0.29792074959875355558e-1_f64) * t47978 - F::cast_from(0.29792074959875355558e-1_f64) * t47980 - F::cast_from(0.29792074959875355558e-1_f64) * t47984;
    t50871
}
