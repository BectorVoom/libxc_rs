//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 987/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk987(t12255: f64, t14428: f64, t2508: f64, t2927: f64, t2963: f64, t3732: f64, t43095: f64, t43098: f64, t44898: f64, t44901: f64, t44905: f64, t44912: f64, t44916: f64, t44921: f64, t44924: f64, t44927: f64, t44928: f64, t44931: f64, t44933: f64, t44936: f64, t44938: f64, t7137: f64) -> f64 {
    let t50421 = -0.34180116578409885705e-2_f64 * t43095 + 0.51270174867614828559e-2_f64 * t43098 + t44898 + 0.15381052460284448567e-1_f64 * t2508 * t3732 * t2927 - 0.61524209841137794269e-1_f64 * t7137 * t14428 - t44901 + t44905 - t44912 - t44916 - t44921 + t44924 - t44927 + 0.1281754371690370714e-2_f64 * t44928 - 0.46143157380853345702e-1_f64 * t2508 * t12255 * t2963 + t44931 - t44933 - t44936 + t44938;
    t50421
}
