//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 347/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk347<F: Float>(t1151: F, t421: F, t1161: F, t1156: F, t560: F, t561: F, t1168: F, t1165: F, t1173: F, t420: F, t419: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1689 = F::new(1.28) * t1151 * t421;
    let t1690 = t421 * t1161;
    let t1692 = F::new(1.28) * t1156 * t1690;
    let t1693 = F::cast_from(1.6737293645052418_f64) * t560;
    let t1694 = F::cast_from(1.1128286385316888_f64) * t561;
    let t1695 = F::cast_from(1.6970672967450864_f64) * t1168;
    let t1696 = -t1165 - t1693 + t1694 + t1695 + t1173;
    let t1697 = t1696 * t420;
    let t1700 = t419 * t419;
    let t1701 = F::new(1.0) / t1700;
    (t1689, t1690, t1692, t1693, t1694, t1695, t1696, t1697, t1700, t1701)
}
