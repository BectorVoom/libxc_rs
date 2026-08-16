//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 681/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk681(t1161: f64, t1701: f64, t1705: f64, t6360: f64, t421: f64, t4830: f64, t1156: f64, t1696: f64, t1700: f64, t419: f64, t1151: f64, t418: f64) -> (f64, f64, f64, f64, f64) {
    let t6361 = t1701 * t1161;
    let t6362 = t6361 * t1705;
    let t6363 = t6360 * t6362;
    let t6365 = t421 * t4830;
    let t6367 = 1.28_f64 * t1156 * t6365;
    let t6376 = t1696 * t1701;
    let t6381 = 1.0_f64 / t1700 / t419;
    let t6403 = t1151 * t418;
    (t6363, t6367, t6376, t6381, t6403)
}
