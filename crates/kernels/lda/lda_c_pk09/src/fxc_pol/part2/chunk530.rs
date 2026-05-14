//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 530/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk530<F: Float>(t3998: F, t891: F, t3997: F, t3161: F, t61: F, t96: F, t3523: F, t55: F, t130: F, t3104: F, t893: F, t132: F, t3677: F, t161: F, t2983: F, t340: F, t838: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3999 = t891 * t3998;
    let t4000 = t3997 * t3999;
    let t4001 = 5.40024514194619 * t4000;
    let t4002 = t61 * t3161;
    let t4003 = t96 * t4002;
    let t4004 = t3523 * t4003;
    let t4005 = 44.15969676259812 * t4004;
    let t4006 = t55 * t55;
    let t4007 = 1.0 / t4006;
    let t4008 = t130 * t4007;
    let t4021 = t3104 * t893;
    let t4023 = t3677 * t132;
    let t4028 = t161 * t2983;
    let t4030 = t838 * t340;
    (t4000, t4001, t4004, t4005, t4007, t4008, t4021, t4023, t4028, t4030)
}
