//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 923/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk923(t1302: f64, t9758: f64, t306: f64, t1292: f64, t1629: f64, t2587: f64, t311: f64, t4756: f64, t4759: f64, t4765: f64, t4769: f64, t4782: f64, t9579: f64, t9582: f64, t9585: f64, t9590: f64, t9592: f64, t9596: f64, t9600: f64, t9603: f64, t9606: f64, t9609: f64, t9612: f64, t9616: f64, t9619: f64) -> (f64, f64) {
    let t9759 = t9758 * t1302;
    let t9760 = t9759 * t306;
    let t9763 = 2.9824072957409817_f64 * t4756 - t4765 - t4769 + 2.9824072957409817_f64 * t4759 * t2587 - 38.978347549160304_f64 * t9579 * t9582 + 2.9824072957409817_f64 * t9585 * t1629 - 0.15277772349540736_f64 * t9590 * t9592 + 5.9648145914819635_f64 * t9596 * t9592 + 2.9824072957409817_f64 * t9600 + t4782 - 1.8805371096875316_f64 * t9603 * t1292 + 1.8805371096875316_f64 * t9606 * t311 - 19.489173774580152_f64 * t9609 * t1292 + 19.489173774580152_f64 * t9612 * t311 - 1.8805371096875316_f64 * t9616 - 19.489173774580152_f64 * t9619 + 19.489173774580152_f64 * t9760 * t311;
    (t9759, t9763)
}
