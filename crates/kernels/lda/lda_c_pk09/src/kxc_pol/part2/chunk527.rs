//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 527/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk527<F: Float>(t3163: F, t3853: F, t174: F, t759: F, t3330: F, t3332: F, t3339: F, t769: F, t10: F, t88: F, t119: F, t703: F, t3172: F, t3767: F, t3194: F, t3820: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3855 = 7.5221484387501265 * t3853 * t3163;
    let t3857 = 1.0 / t759 / t174;
    let t3870 = 1.5625 * t3330;
    let t3871 = 2.0833333333333335 * t3332;
    let t3873 = 0.3472222222222222 * t3339;
    let t3886 = t759 * t759;
    let t3888 = 1.0 / t3886 / t174;
    let t3890 = t769 * t769;
    let t3891 = 1.0 / t3890;
    let t3893 = t3891 * t88 * t10;
    let t3897 = t703 * t119;
    let t3906 = t3767 * t3172;
    let t3908 = t3820 * t3194;
    (t3855, t3857, t3870, t3871, t3873, t3888, t3893, t3897, t3906, t3908)
}
