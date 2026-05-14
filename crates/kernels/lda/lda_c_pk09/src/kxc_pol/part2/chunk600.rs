//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 600/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk600<F: Float>(t5212: F, t5068: F, t130: F, t1592: F, t93: F, t1435: F, t1565: F, t1581: F, t5039: F, t5161: F, t5045: F, t5190: F, t5208: F, t307: F, t4767: F, t328: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5923 = 1.5323028051206833 * t5212;
    let t5925 = 0.06792226392670653 * t5068;
    let t5932 = t130 * t1592;
    let t5933 = t93 * t5932;
    let t5939 = t1565 * t1435;
    let t5941 = t1581 * t1435;
    let t5952 = 0.15282509383508946 * t5039;
    let t5956 = 1.02153520341379 * t5161;
    let t5965 = 0.10188339589005964 * t5045;
    let t5966 = 0.08512793361781583 * t5190;
    let t5971 = 0.7661514025603425 * t5208;
    let t5972 = 0.7661514025603425 * t5212;
    let t5974 = 0.033961131963353215 * t5068;
    let t5982 = 2.0 / 27.0 * t307 * t4767;
    let t5984 = 2.0 / 27.0 * t328 * t4767;
    (t5923, t5925, t5933, t5939, t5941, t5952, t5956, t5965, t5966, t5971, t5972, t5974, t5982, t5984)
}
