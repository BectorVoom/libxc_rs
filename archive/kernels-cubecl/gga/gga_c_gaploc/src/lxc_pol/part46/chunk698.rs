//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 698/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk698<F: Float>(t12456: F, t12906: F, t12909: F, t12911: F, t12912: F, t12916: F, t12921: F, t12924: F, t12928: F, t12929: F, t12930: F, t12931: F) -> F {
    let t12932 = F::cast_from(0.59584149919750711116e-1_f64) * t12456;
    let t12933 = -F::cast_from(0.92023022289409799224e1_f64) * t12906 + t12909 + t12911 + F::cast_from(0.71500979903700853338e0_f64) * t12912 - F::cast_from(0.13803453343411469884e2_f64) * t12916 - t12921 + t12924 - t12928 - t12929 + t12930 - t12931 + t12932;
    t12933
}
