//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 904/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk904<F: Float>(t3319: F, t8138: F, t8930: F, t1125: F, t427: F, t426: F, t1250: F, t47: F, t1332: F, t52: F, t411: F, t717: F, t732: F) -> (F, F, F, F, F, F) {
    let t8936 = F::cast_from(1.6239027777777777_f64) * param_hyb_omega_0 * t8138 * t3319 * t8930;
    let t8939 = t1125 * t427;
    let t8940 = t426 * t8939;
    let t8949 = F::new(1.0) / t47 / t1250;
    let t8962 = F::new(1.0) / t52 / t1332;
    let t8998 = t732 * t717 * t411;
    (t8936, t8939, t8940, t8949, t8962, t8998)
}
