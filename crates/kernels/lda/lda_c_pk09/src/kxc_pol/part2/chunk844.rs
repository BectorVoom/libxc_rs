//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 844/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk844<F: Float>(t133: F, t8231: F, t1062: F, t2354: F, t721: F, t119: F, t1011: F, t131: F, t2222: F, t4567: F, t4570: F, t4572: F, t4574: F, t4576: F, t4584: F, t4590: F, t4595: F, t713: F, t727: F) -> (F, F) {
    let t8639 = t133 * t8231;
    let t8648 = t2354 * t1062;
    let t8649 = t8648 * t721;
    let t8651 = t2354 * t119;
    let t8654 = F::cast_from(3.7610742193750633_f64) * t4567 + t4570 + F::cast_from(0.6268457032291772_f64) * t4572 - F::cast_from(2.507382812916709_f64) * t4574 - F::cast_from(2.507382812916709_f64) * t4576 - F::cast_from(0.5923479790153209_f64) * t727 * t131 * t8639 - F::cast_from(4.738783832122567_f64) * t4584 + F::cast_from(1.1846959580306418_f64) * t4590 - F::cast_from(3.159189221415045_f64) * t4595 - F::cast_from(2.9824072957409817_f64) * t2222 * t1011 - F::cast_from(19.489173774580152_f64) * t8649 - F::cast_from(19.489173774580152_f64) * t8651 * t713;
    (t8651, t8654)
}
