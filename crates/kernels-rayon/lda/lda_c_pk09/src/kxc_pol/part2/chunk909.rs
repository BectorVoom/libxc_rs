//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 909/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk909(t2455: f64, t4785: f64, t1151: f64, t2448: f64, t1161: f64, t1156: f64, t5: f64, t2962: f64, t2964: f64, t4837: f64, t4842: f64, t271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9637 = t4785 * t2455;
    let t9643 = t1151 * t2448;
    let t9645 = t2448 * t1161;
    let t9646 = t1156 * t9645;
    let t9648 = 2.8538608299684327_f64 * t5;
    let t9649 = 1.1218014519471058_f64 * t2962;
    let t9650 = 8.429687805830326_f64 * t2964;
    let t9651 = 6.964128765746976_f64 * t4837;
    let t9652 = t9648 - t9649 - t9650 + t9651 - t4842;
    let t9653 = t9652 * t271;
    (t9637, t9643, t9646, t9648, t9649, t9650, t9651, t9653)
}
