//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 643/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk643<F: Float>(t451: F, t6586: F, t309: F, t454: F, t4993: F, t462: F, t1240: F, t1671: F, t2056: F, t471: F, t2045: F, t4977: F, t2040: F, t6791: F, t6803: F, t2037: F, t6253: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7004 = t451 * t6586;
    let t7006 = t309 * t454 * t4993;
    let t7008 = t7004 * t7006 / 3.0;
    let t7013 = t462 * t6586;
    let t7015 = t7013 * t7006 / 3.0;
    let t7017 = t309 * t1671 * t1240;
    let t7019 = t2056 * t7017 / 9.0;
    let t7024 = t471 * t6586;
    let t7026 = t7024 * t7006 / 3.0;
    let t7028 = t2045 * t7017 / 9.0;
    let t7030 = t309 * t454 * t4977;
    let t7032 = t2040 * t7030 / 6.0;
    let t7041 = 0.037002892246025966 * t6791;
    let t7045 = 0.14975624337724558 * t6803;
    let t7049 = t2037 * t6253;
    (t7008, t7015, t7017, t7019, t7026, t7028, t7030, t7032, t7041, t7045, t7049)
}
