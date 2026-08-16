//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 987/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk987<F: Float>(t12255: F, t14428: F, t2508: F, t2927: F, t2963: F, t3732: F, t43095: F, t43098: F, t44898: F, t44901: F, t44905: F, t44912: F, t44916: F, t44921: F, t44924: F, t44927: F, t44928: F, t44931: F, t44933: F, t44936: F, t44938: F, t7137: F) -> F {
    let t50421 = -F::cast_from(0.34180116578409885705e-2_f64) * t43095 + F::cast_from(0.51270174867614828559e-2_f64) * t43098 + t44898 + F::cast_from(0.15381052460284448567e-1_f64) * t2508 * t3732 * t2927 - F::cast_from(0.61524209841137794269e-1_f64) * t7137 * t14428 - t44901 + t44905 - t44912 - t44916 - t44921 + t44924 - t44927 + F::cast_from(0.1281754371690370714e-2_f64) * t44928 - F::cast_from(0.46143157380853345702e-1_f64) * t2508 * t12255 * t2963 + t44931 - t44933 - t44936 + t44938;
    t50421
}
