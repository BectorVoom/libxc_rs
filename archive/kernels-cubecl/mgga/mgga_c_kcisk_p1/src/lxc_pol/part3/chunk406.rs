//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 406/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk406<F: Float>(t2977: F, t73: F, t88: F, t2950: F, t45: F, t857: F, t890: F, t98: F, t896: F, t898: F, t2958: F, t2960: F, t2962: F, t2967: F, t2969: F, t2971: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2978 = F::cast_from(1.0_f64) / t2977;
    let t2979 = t73 * t2978;
    let t2980 = t88 * t88;
    let t2981 = F::cast_from(1.0_f64) / t2980;
    let t2982 = t2950 * t2981;
    let t2989 = t45 * t857;
    let t2993 = t890 * t98;
    let t2994 = F::cast_from(1.0_f64) / t2993;
    let t2995 = t896 * t896;
    let t2997 = t2994 * t2995 * t898;
    let t3006 = -F::cast_from(0.57538888888888888889e0_f64) * t2958 + F::cast_from(0.11507777777777777778e1_f64) * t2960 + F::cast_from(0.40256666666666666667e0_f64) * t2962 + F::cast_from(0.366775e-1_f64) * t2967 + F::cast_from(0.73355e-1_f64) * t2969 + F::cast_from(0.137975e0_f64) * t2971;
    (t2978, t2979, t2980, t2981, t2982, t2989, t2994, t2995, t2997, t3006)
}
