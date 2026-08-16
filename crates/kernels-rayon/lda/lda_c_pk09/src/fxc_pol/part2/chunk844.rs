//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 844/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk844(t133: f64, t8231: f64, t1062: f64, t2354: f64, t721: f64, t119: f64, t1011: f64, t131: f64, t2222: f64, t4567: f64, t4570: f64, t4572: f64, t4574: f64, t4576: f64, t4584: f64, t4590: f64, t4595: f64, t713: f64, t727: f64) -> (f64, f64) {
    let t8639 = t133 * t8231;
    let t8648 = t2354 * t1062;
    let t8649 = t8648 * t721;
    let t8651 = t2354 * t119;
    let t8654 = 3.7610742193750633_f64 * t4567 + t4570 + 0.6268457032291772_f64 * t4572 - 2.507382812916709_f64 * t4574 - 2.507382812916709_f64 * t4576 - 0.5923479790153209_f64 * t727 * t131 * t8639 - 4.738783832122567_f64 * t4584 + 1.1846959580306418_f64 * t4590 - 3.159189221415045_f64 * t4595 - 2.9824072957409817_f64 * t2222 * t1011 - 19.489173774580152_f64 * t8649 - 19.489173774580152_f64 * t8651 * t713;
    (t8651, t8654)
}
