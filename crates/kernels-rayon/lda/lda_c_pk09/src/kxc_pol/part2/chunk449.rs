//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 449/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk449(t1069: f64, t1071: f64, t1073: f64, t1100: f64, t1108: f64, t1114: f64, t1115: f64, t1116: f64, t1117: f64, t2171: f64, t2175: f64, t2179: f64, t2248: f64, t2252: f64, t2256: f64, t2260: f64, t2264: f64, t2394: f64, t98: f64) -> f64 {
    let t2405 = -t2394 * t98 / 6.0_f64 - 0.10237773105191754_f64 * t2171 - 0.10237773105191754_f64 * t2175 + t1069 + t1071 - t1073 - t1100 + t1108 - 0.14975624337724558_f64 * t2248 - 0.14975624337724558_f64 * t2252 + 0.10237773105191754_f64 * t2179 - 0.01233429741534199_f64 * t2256 - 0.01233429741534199_f64 * t2260 + 0.01233429741534199_f64 * t2264 - t1114 - t1115 - t1116 - t1117;
    t2405
}
