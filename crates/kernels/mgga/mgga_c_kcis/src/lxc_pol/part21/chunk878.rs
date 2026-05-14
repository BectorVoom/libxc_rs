//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 878/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk878<F: Float>(t14622: F, t3227: F, t1092: F, t1017: F, t342: F, t86: F, t1130: F, t1767: F, t2815: F, t9410: F, t1662: F, t9517: F, t3200: F, t4802: F, t9425: F, t13132: F, t4555: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14623 = t3227 * t14622;
    let t14624 = t1092 * t14623;
    let t14627 = t86 * t1017 * t342;
    let t14628 = t1130 * t1767;
    let t14629 = t14628 * t2815;
    let t14630 = t9410 * t14629;
    let t14631 = t14627 * t14630;
    let t14633 = t1662 * t2815;
    let t14634 = t9517 * t14633;
    let t14635 = t3200 * t14634;
    let t14638 = t9425 * t4802;
    let t14640 = t4555 * t13132;
    (t14624, t14627, t14628, t14629, t14631, t14633, t14635, t14638, t14640)
}
