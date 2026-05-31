//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1274/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1274<F: Float>(t1325: F, t5237: F, t7701: F, t1997: F, t6198: F, t18438: F, t18444: F, t18446: F, t18449: F, t1318: F, t4776: F, t549: F, t7408: F) -> (F, F, F, F, F, F, F) {
    let t22856 = t1325 * t5237 * t7701;
    let t22857 = F::cast_from(16.0_f64) / F::cast_from(27.0_f64) * t22856;
    let t22859 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t6198 * t1997;
    let t22860 = F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t18438;
    let t22861 = F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t18444;
    let t22862 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t18446;
    let t22863 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t18449;
    let t22868 = F::cast_from(64.0_f64) / F::cast_from(81.0_f64) * t1318 * t4776 * t7408 * t549;
    (t22857, t22859, t22860, t22861, t22862, t22863, t22868)
}
