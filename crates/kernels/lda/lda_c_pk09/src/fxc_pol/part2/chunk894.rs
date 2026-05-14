//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 894/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk894<F: Float>(t51: F, t2715: F, t9731: F, t11004: F, t11013: F, t11016: F, t11020: F, t11024: F, t1204: F, t1207: F, t1713: F, t2711: F, t2716: F, t630: F, t6409: F, t11007: F, t1719: F, t2719: F, t425: F, zeta_threshold: F) -> (F,) {
    let t52 = t51 <= zeta_threshold;
    let t11027 = t2715 * t9731;
    let t11030 = t11004 * t1204 + t2711 * t630 * t1207 + t11013 - t11016 + 1.28 * t6409 * t2716 - 1.28 * t1713 * t11020 - 2.56 * t1713 * t11024 - 1.28 * t1713 * t11027;
    let t11033 = piecewise3(t52, t11007, t11030 * t425 + t1719 * t2719);
    (t11033,)
}
