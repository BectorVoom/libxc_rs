//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 655/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk655<F: Float>(t3165: F, t343: F, t858: F, t867: F, t866: F, t3131: F, t3139: F, t875: F, t2168: F, t2143: F, t2165: F, t2207: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3166 = t3165 * t343;
    let t3167 = t858 * t3166;
    let t3168 = t867 * t3167;
    let t3170 = t866 * t3168 / F::cast_from(96.0_f64);
    let t3172 = t3139 * t3131 * t875;
    let t3174 = t2168 * t3172 / F::cast_from(96.0_f64);
    let t3175 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t2143;
    let t3176 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t2165;
    let t3177 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2207;
    (t3166, t3167, t3168, t3170, t3172, t3174, t3175, t3176, t3177)
}
