//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 724/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk724<F: Float>(t6501: F, t6505: F, t6522: F, t6319: F, t6325: F, t6547: F, t6464: F, t1672: F, t1898: F, t1836: F, t6790: F, t1853: F, t6488: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7362 = F::new(6.25) * t6501;
    let t7363 = F::new(6.25) * t6505;
    let t7367 = F::new(8.333333333333334) * t6522;
    let t7371 = F::new(1.2466946262544771) * t6319;
    let t7378 = F::new(0.8311297508363181) * t6325;
    let t7379 = F::new(0.6944444444444444) * t6547;
    let t7384 = F::new(0.2770432502787727) * t6464;
    let t7395 = t1898 * t1672;
    let t7400 = F::new(7.108175748183851) * t1836 * t6790;
    let t7402 = F::new(1.6183441301295518) * t1853 * t6488;
    (t7362, t7363, t7367, t7371, t7378, t7379, t7384, t7395, t7400, t7402)
}
