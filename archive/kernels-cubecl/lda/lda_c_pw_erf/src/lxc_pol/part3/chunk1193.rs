//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1193/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1193<F: Float>(t10030: F, t4476: F, t2070: F, t807: F, t185: F, t3679: F, t795: F, t834: F, t211: F, t548: F, t812: F, t10632: F) -> (F, F, F, F, F, F) {
    let t14041 = t10030 * t4476;
    let t14042 = F::cast_from(32.0_f64) / F::cast_from(45.0_f64) * t14041;
    let t14043 = t2070 * t807;
    let t14044 = t185 * t14043;
    let t14045 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t14044;
    let t14047 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t795 * t3679;
    let t14048 = t2070 * t834;
    let t14049 = t211 * t14048;
    let t14050 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t14049;
    let t14052 = t548 * t2070 * t812;
    let t14053 = F::cast_from(32.0_f64) / F::cast_from(405.0_f64) * t14052;
    let t14054 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t10632;
    (t14042, t14045, t14047, t14050, t14053, t14054)
}
