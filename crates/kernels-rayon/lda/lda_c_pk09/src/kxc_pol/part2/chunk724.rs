//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 724/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk724(t6501: f64, t6505: f64, t6522: f64, t6319: f64, t6325: f64, t6547: f64, t6464: f64, t1672: f64, t1898: f64, t1836: f64, t6790: f64, t1853: f64, t6488: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7362 = 6.25_f64 * t6501;
    let t7363 = 6.25_f64 * t6505;
    let t7367 = 8.333333333333334_f64 * t6522;
    let t7371 = 1.2466946262544771_f64 * t6319;
    let t7378 = 0.8311297508363181_f64 * t6325;
    let t7379 = 0.6944444444444444_f64 * t6547;
    let t7384 = 0.2770432502787727_f64 * t6464;
    let t7395 = t1898 * t1672;
    let t7400 = 7.108175748183851_f64 * t1836 * t6790;
    let t7402 = 1.6183441301295518_f64 * t1853 * t6488;
    (t7362, t7363, t7367, t7371, t7378, t7379, t7384, t7395, t7400, t7402)
}
