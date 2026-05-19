//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1300/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1300<F: Float>(t10257: F, t10259: F, t12371: F, t16379: F, t21910: F, t21911: F, t21912: F, t21913: F, t21914: F, t21917: F, t21920: F, t21921: F, t21922: F, t21923: F, t8016: F, t8018: F, t8023: F) -> F {
    let t50812 = F::cast_from(0.79007158810260824916e-1_f64) * t12371 - F::new(48.0) * t10257 - F::new(48.0) * t10259 - F::cast_from(0.70178680769462448852e1_f64) * t8016 - F::cast_from(0.2077890707925103596e3_f64) * t8018 + F::cast_from(0.14035736153892489771e2_f64) * t8023 + t16379 - t21910 - t21911 - t21912 + t21913 + t21914 + t21917 - t21920 - t21921 - t21922 - t21923;
    t50812
}
