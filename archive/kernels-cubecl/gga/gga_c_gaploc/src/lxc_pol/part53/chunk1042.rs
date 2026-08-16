//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1042/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1042<F: Float>(t40591: F, t40596: F, t40599: F, t42974: F, t42978: F, t42980: F, t42984: F, t42985: F, t42988: F, t42991: F, t42998: F, t43006: F, t43014: F, t43017: F, t43019: F, t43023: F, t47616: F, t47619: F) -> F {
    let t51007 = -t42974 - t42978 - t42980 - F::cast_from(0.44861403009162974988e-2_f64) * t40591 + F::cast_from(0.76905262301422242835e-2_f64) * t40596 - F::cast_from(0.38452631150711121417e-2_f64) * t40599 + t42984 + F::cast_from(0.1281754371690370714e-2_f64) * t42985 + F::cast_from(0.1281754371690370714e-2_f64) * t42988 + F::cast_from(0.1281754371690370714e-2_f64) * t42991 - t47616 + t47619 - t42998 + t43006 - t43014 - t43017 + t43019 - t43023;
    t51007
}
