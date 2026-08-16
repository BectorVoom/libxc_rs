//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 348/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk348(t1701: f64, t418: f64, t560: f64, t561: f64, t1181: f64, t1184: f64) -> (f64, f64, f64, f64) {
    let t1702 = t418 * t1701;
    let t1703 = 2.9466786129040563_f64 * t560;
    let t1704 = 2.043763671738964_f64 * t561;
    let t1705 = -t1181 - t1703 + t1704 + t1184;
    (t1702, t1703, t1704, t1705)
}
