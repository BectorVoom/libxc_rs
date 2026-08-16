//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1000/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1000(t347: f64, t9602: f64, t1314: f64, t1287: f64, t10572: f64, t130: f64, t10774: f64, t10777: f64, t10780: f64, t10786: f64, t10792: f64, t10795: f64, t10800: f64, t10803: f64, t10808: f64, t1292: f64, t1383: f64, t1388: f64, t1476: f64, t1481: f64, t2517: f64, t2587: f64, t5585: f64, t5587: f64, t5590: f64, t5716: f64, t93: f64) -> f64 {
    let t10810 = t347 * t9602;
    let t10815 = t1314 * t9602;
    let t10816 = t10815 * t1287;
    let t10818 = t130 * t10572;
    let t10822 = 0.9941357652469939_f64 * t10774 - 4.937333717448355_f64 * t10777 + 3.5540878740919255_f64 * t1388 * t93 * t10780 + 2.427516195194328_f64 * t1383 * t2587 + 3.5540878740919255_f64 * t1388 * t93 * t10786 - 14.216351496367702_f64 * t10792 + 14.216351496367702_f64 * t1476 * t10795 - 21.324527244551554_f64 * t1481 * t10800 - 2.427516195194328_f64 * t10803 - 1.8805371096875316_f64 * t5585 + 19.489173774580152_f64 * t5587 - 19.489173774580152_f64 * t5590 - 0.7380249726277691_f64 * t10808 + 3.7610742193750633_f64 * t10810 * t1292 - 1.8805371096875316_f64 * t5716 * t2517 + 3.7610742193750633_f64 * t10816 - 1.7770439370459628_f64 * t1388 * t93 * t10818;
    t10822
}
