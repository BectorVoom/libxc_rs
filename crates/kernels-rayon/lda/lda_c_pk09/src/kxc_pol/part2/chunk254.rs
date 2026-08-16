//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 254/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk254(t1075: f64, t1120: f64, t79: f64, t137: f64, t1091: f64, t143: f64, t155: f64, t179: f64, t192: f64, t200: f64, t205: f64, t709: f64, t713: f64, t756: f64, t80: f64, t946: f64, t949: f64, t953: f64, t959: f64, t976: f64, t98: f64, t982: f64, t986: f64) -> (f64, f64, f64, f64, f64) {
    let t1121 = t1075 + t1120;
    let t1123 = t79 * t79;
    let t1124 = 1.0_f64 / t1123;
    let t1125 = t1124 * t137;
    let t1127 = -2.2140749178833072_f64 * t192 * t756 + 18.635258017632964_f64 * t179 * t756 - t946 + 2.3693919160612835_f64 * t205 * t949 - 2.3693919160612835_f64 * t205 * t953 + t959 + 2.427516195194328_f64 * t200 * t713 - 19.489173774580152_f64 * t155 * t713 - 19.489173774580152_f64 * t155 * t709 + 19.489173774580152_f64 * t976 * t98 - t982 + t986 + 3.7610742193750633_f64 * t143 * t713 + 3.7610742193750633_f64 * t143 * t709 + 2.427516195194328_f64 * t200 * t709 + t80 * t1121 - t1125 * t1091;
    (t1121, t1123, t1124, t1125, t1127)
}
