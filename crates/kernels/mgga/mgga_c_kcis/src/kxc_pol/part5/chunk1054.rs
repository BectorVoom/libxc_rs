//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1054/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1054<F: Float>(t6432: F, t978: F, t1767: F, t829: F, t4566: F, t14381: F, t4554: F, t3182: F, t6555: F, t1021: F, t1092: F, t4995: F, t4999: F, t4994: F, t13181: F, t1713: F) -> (F, F, F, F, F, F) {
    let t19674 = t6432 * t978;
    let t19679 = t1767 * t829;
    let t19680 = t4566 * t19679;
    let t19681 = t14381 * t19680;
    let t19682 = t4554 * t19681;
    let t19684 = t3182 * t6555;
    let t19685 = t1021 * t19684;
    let t19686 = t1092 * t19685;
    let t19688 = t4999 * t4995;
    let t19689 = t4994 * t19688;
    let t19691 = t13181 * t1713;
    (t19674, t19679, t19682, t19686, t19689, t19691)
}
