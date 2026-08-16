//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 923/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk923<F: Float>(t1302: F, t9758: F, t306: F, t1292: F, t1629: F, t2587: F, t311: F, t4756: F, t4759: F, t4765: F, t4769: F, t4782: F, t9579: F, t9582: F, t9585: F, t9590: F, t9592: F, t9596: F, t9600: F, t9603: F, t9606: F, t9609: F, t9612: F, t9616: F, t9619: F) -> (F, F) {
    let t9759 = t9758 * t1302;
    let t9760 = t9759 * t306;
    let t9763 = F::cast_from(2.9824072957409817_f64) * t4756 - t4765 - t4769 + F::cast_from(2.9824072957409817_f64) * t4759 * t2587 - F::cast_from(38.978347549160304_f64) * t9579 * t9582 + F::cast_from(2.9824072957409817_f64) * t9585 * t1629 - F::cast_from(0.15277772349540736_f64) * t9590 * t9592 + F::cast_from(5.9648145914819635_f64) * t9596 * t9592 + F::cast_from(2.9824072957409817_f64) * t9600 + t4782 - F::cast_from(1.8805371096875316_f64) * t9603 * t1292 + F::cast_from(1.8805371096875316_f64) * t9606 * t311 - F::cast_from(19.489173774580152_f64) * t9609 * t1292 + F::cast_from(19.489173774580152_f64) * t9612 * t311 - F::cast_from(1.8805371096875316_f64) * t9616 - F::cast_from(19.489173774580152_f64) * t9619 + F::cast_from(19.489173774580152_f64) * t9760 * t311;
    (t9759, t9763)
}
