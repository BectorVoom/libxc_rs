//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 566/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk566<F: Float>(t3339: F, t3330: F, t3444: F, t3453: F, t2971: F, t983: F, t2974: F, t141: F, t154: F, t119: F, t975: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3794 = F::new(0.010056629776875343) * t3339;
    let t3803 = F::new(0.04525483399593904) * t3330;
    let t3810 = F::new(0.6806222787477182) * t3444;
    let t3812 = F::new(1.8149927433272484) * t3453;
    let t3820 = t983 * t2971;
    let t3821 = t3820 * t2974;
    let t3823 = t141 * t2971;
    let t3826 = t154 * t2971;
    let t3829 = t975 * t119;
    let t3835 = t973 * t973;
    (t3794, t3803, t3810, t3812, t3820, t3821, t3823, t3826, t3829, t3835)
}
