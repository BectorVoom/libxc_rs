//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1000/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1000<F: Float>(t347: F, t9602: F, t1314: F, t1287: F, t10572: F, t130: F, t10774: F, t10777: F, t10780: F, t10786: F, t10792: F, t10795: F, t10800: F, t10803: F, t10808: F, t1292: F, t1383: F, t1388: F, t1476: F, t1481: F, t2517: F, t2587: F, t5585: F, t5587: F, t5590: F, t5716: F, t93: F) -> F {
    let t10810 = t347 * t9602;
    let t10815 = t1314 * t9602;
    let t10816 = t10815 * t1287;
    let t10818 = t130 * t10572;
    let t10822 = F::new(0.9941357652469939) * t10774 - F::new(4.937333717448355) * t10777 + F::new(3.5540878740919255) * t1388 * t93 * t10780 + F::new(2.427516195194328) * t1383 * t2587 + F::new(3.5540878740919255) * t1388 * t93 * t10786 - F::new(14.216351496367702) * t10792 + F::new(14.216351496367702) * t1476 * t10795 - F::new(21.324527244551554) * t1481 * t10800 - F::new(2.427516195194328) * t10803 - F::new(1.8805371096875316) * t5585 + F::new(19.489173774580152) * t5587 - F::new(19.489173774580152) * t5590 - F::new(0.7380249726277691) * t10808 + F::new(3.7610742193750633) * t10810 * t1292 - F::new(1.8805371096875316) * t5716 * t2517 + F::new(3.7610742193750633) * t10816 - F::new(1.7770439370459628) * t1388 * t93 * t10818;
    t10822
}
