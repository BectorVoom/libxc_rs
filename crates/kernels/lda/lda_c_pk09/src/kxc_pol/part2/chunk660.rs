//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 660/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk660<F: Float>(t5068: F, t130: F, t1592: F, t93: F, t1435: F, t1565: F, t1581: F, t5039: F, t5161: F, t5045: F, t5190: F, t5208: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5925 = F::cast_from(0.06792226392670653_f64) * t5068;
    let t5932 = t130 * t1592;
    let t5933 = t93 * t5932;
    let t5939 = t1565 * t1435;
    let t5941 = t1581 * t1435;
    let t5952 = F::cast_from(0.15282509383508946_f64) * t5039;
    let t5956 = F::cast_from(1.02153520341379_f64) * t5161;
    let t5965 = F::cast_from(0.10188339589005964_f64) * t5045;
    let t5966 = F::cast_from(0.08512793361781583_f64) * t5190;
    let t5971 = F::cast_from(0.7661514025603425_f64) * t5208;
    (t5925, t5933, t5939, t5941, t5952, t5956, t5965, t5966, t5971)
}
