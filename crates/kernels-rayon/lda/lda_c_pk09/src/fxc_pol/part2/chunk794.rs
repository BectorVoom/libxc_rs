//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 794/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk794(t2288: f64, t650: f64, t825: f64, t96: f64, t1011: f64, t143: f64, t155: f64, t164: f64, t205: f64, t2198: f64, t2202: f64, t2419: f64, t713: f64, t7706: f64, t7776: f64, t7784: f64, t7786: f64, t7790: f64, t7792: f64, t7948: f64, t7952: f64, t7956: f64, t7962: f64, t933: f64, t98: f64) -> f64 {
    let t7967 = t96 * t650 * t2288 * t825;
    let t7970 = 0.6268457032291772_f64 * t933 * t2198 + 3.7610742193750633_f64 * t143 * t7776 + 3.7610742193750633_f64 * t143 * t7706 + 0.6268457032291772_f64 * t933 * t2202 + 2.427516195194328_f64 * t7784 * t7786 + 2.427516195194328_f64 * t7790 + 2.427516195194328_f64 * t7792 * t713 - 2.427516195194328_f64 * t7948 * t98 - 2.3693919160612835_f64 * t205 * t7952 - 2.3693919160612835_f64 * t205 * t7956 + 1.8805371096875316_f64 * t2419 * t1011 - 19.489173774580152_f64 * t155 * t7962 - 0.04115066352984959_f64 * t164 * t7967;
    t7970
}
