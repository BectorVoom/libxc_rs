//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3286/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3286<F: Float>(t5722: F, t74835: F, t1357: F, t23043: F, t689: F, t47561: F, t47907: F, t47920: F, t47932: F, t47938: F, t47942: F, t47945: F, t47948: F, t47953: F, t49468: F, t73712: F, t74733: F, t74744: F) -> F {
    let t86296 = t74835 * t5722;
    let t86300 = t689 * t1357 * t23043;
    let t86308 = F::cast_from(0.19514881078765566038e-2_f64) * t47907 - F::cast_from(0.34697458558045176418e-2_f64) * t73712 - F::cast_from(0.29272321618148349057e-1_f64) * t86296 - F::cast_from(0.13878983423218070567e-1_f64) * t47920 + F::cast_from(0.54878743191129263322e-2_f64) * t86300 - F::cast_from(0.69394917116090352834e-2_f64) * t74733 + F::cast_from(0.13878983423218070567e-1_f64) * t47932 - F::cast_from(0.65854491829355115984e-1_f64) * t74744 + F::cast_from(0.78059524315062264151e-2_f64) * t47938 + F::cast_from(0.19514881078765566038e-2_f64) * t47942 - t47945 + t47948 + t47953 + t47561 - F::cast_from(0.51220160311720645768e-1_f64) * t49468;
    t86308
}
