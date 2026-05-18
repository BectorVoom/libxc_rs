//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 949/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk949<F: Float>(t4524: F, t643: F, t638: F, t3957: F, t4549: F, t3960: F, t3966: F, t1122: F, t2142: F, t30: F, t3963: F, t8685: F, t8692: F, t8693: F, t8723: F, t8724: F, t8727: F, t8729: F, t8733: F, t8737: F) -> F {
    let t11110 = t643 * t4524;
    let t11112 = t638 * t4524;
    let t11113 = F::new(12.0) * t11112;
    let t11115 = t4549 * t3957;
    let t11117 = t4549 * t3960;
    let t11119 = t4549 * t3966;
    let t11122 = t2142 * t30 * t1122;
    let t11123 = F::new(0.03253074390090522) * t11122;
    let t11124 = t4549 * t3963;
    let t11126 = -F::new(3076.205657464922) * t8685 + t8692 - F::new(1.7544670867903938) * t8693 - t8723 + F::new(311.68546390226635) * t8724 - F::new(12.0) * t11110 + t11113 + t8727 - F::new(4.0) * t8729 + t8733 - F::new(0.03253074390090522) * t11115 - F::new(0.02168716260060348) * t11117 + F::new(0.4815973313767657) * t11119 + t11123 + F::new(0.01626537195045261) * t11124 - t8737;
    t11126
}
