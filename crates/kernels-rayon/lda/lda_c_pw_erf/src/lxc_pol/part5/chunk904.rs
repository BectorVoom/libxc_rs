//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 904/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk904(t3319: f64, t8138: f64, t8930: f64, t1125: f64, t427: f64, t426: f64, t1250: f64, t47: f64, t1332: f64, t52: f64, t411: f64, t717: f64, t732: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8936 = 1.6239027777777777_f64 * param_hyb_omega_0 * t8138 * t3319 * t8930;
    let t8939 = t1125 * t427;
    let t8940 = t426 * t8939;
    let t8949 = 1.0_f64 / t47 / t1250;
    let t8962 = 1.0_f64 / t52 / t1332;
    let t8998 = t732 * t717 * t411;
    (t8936, t8939, t8940, t8949, t8962, t8998)
}
