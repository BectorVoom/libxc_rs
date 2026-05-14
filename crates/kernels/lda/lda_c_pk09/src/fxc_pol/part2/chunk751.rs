//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 751/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk751<F: Float>(t133: F, t8231: F, t1062: F, t2354: F, t721: F, t119: F, t1011: F, t131: F, t2222: F, t4567: F, t4570: F, t4572: F, t4574: F, t4576: F, t4584: F, t4590: F, t4595: F, t713: F, t727: F) -> (F, F) {
    let t8639 = t133 * t8231;
    let t8648 = t2354 * t1062;
    let t8649 = t8648 * t721;
    let t8651 = t2354 * t119;
    let t8654 = 3.7610742193750633 * t4567 + t4570 + 0.6268457032291772 * t4572 - 2.507382812916709 * t4574 - 2.507382812916709 * t4576 - 0.5923479790153209 * t727 * t131 * t8639 - 4.738783832122567 * t4584 + 1.1846959580306418 * t4590 - 3.159189221415045 * t4595 - 2.9824072957409817 * t2222 * t1011 - 19.489173774580152 * t8649 - 19.489173774580152 * t8651 * t713;
    (t8651, t8654)
}
