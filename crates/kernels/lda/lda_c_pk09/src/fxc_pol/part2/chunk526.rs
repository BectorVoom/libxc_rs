//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 526/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk526<F: Float>(t2971: F, t983: F, t2974: F, t141: F, t154: F, t119: F, t975: F, t973: F, t1062: F, t805: F, t721: F, t3498: F, t984: F, t110: F, t733: F, t142: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3820 = t983 * t2971;
    let t3821 = t3820 * t2974;
    let t3823 = t141 * t2971;
    let t3826 = t154 * t2971;
    let t3829 = t975 * t119;
    let t3835 = t973 * t973;
    let t3836 = 1.0 / t3835;
    let t3843 = t805 * t1062;
    let t3844 = t3843 * t721;
    let t3847 = 5.014765625833418 * t984 * t3498;
    let t3852 = t110 * t733;
    let t3853 = t3852 * t142;
    (t3820, t3821, t3823, t3826, t3829, t3836, t3844, t3847, t3853)
}
