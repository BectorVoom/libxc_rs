//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 683/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk683<F: Float>(t1184: F, t7822: F, t1190: F, t579: F, t839: F, t336: F, t2046: F, t1165: F, t604: F, t930: F, t2068: F, t599: F, t945: F, t1181: F, t7413: F, t2067: F, t3378: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7823 = t7822 * t1184;
    let t7825 = t7822 * t1190;
    let t7827 = t579 * t839;
    let t7828 = t336 * t7827;
    let t7829 = t2046 * t7828;
    let t7832 = t1165 * t604 * t930;
    let t7833 = t2068 * t7832;
    let t7835 = t599 * t945;
    let t7836 = t1181 * t7835;
    let t7837 = t7413 * t7836;
    let t7839 = t3378 * t2067;
    (t7823, t7825, t7828, t7829, t7832, t7833, t7835, t7836, t7837, t7839)
}
