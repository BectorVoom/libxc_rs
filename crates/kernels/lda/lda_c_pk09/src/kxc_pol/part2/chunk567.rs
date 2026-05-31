//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 567/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk567<F: Float>(t3835: F, t1062: F, t805: F, t721: F, t3498: F, t984: F, t110: F, t733: F, t142: F, t3163: F, t174: F, t759: F) -> (F, F, F, F, F) {
    let t3836 = F::cast_from(1.0_f64) / t3835;
    let t3843 = t805 * t1062;
    let t3844 = t3843 * t721;
    let t3847 = F::cast_from(5.014765625833418_f64) * t984 * t3498;
    let t3852 = t110 * t733;
    let t3853 = t3852 * t142;
    let t3855 = F::cast_from(7.5221484387501265_f64) * t3853 * t3163;
    let t3857 = F::cast_from(1.0_f64) / t759 / t174;
    (t3836, t3844, t3847, t3855, t3857)
}
