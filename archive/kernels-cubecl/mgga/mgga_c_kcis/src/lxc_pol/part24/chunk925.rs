//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 925/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk925<F: Float>(t19790: F, t3227: F, t1092: F, t2825: F, t6504: F, t1020: F, t2822: F, t6630: F, t6625: F, t9438: F, t3200: F, t13155: F, t19396: F) -> (F, F, F, F, F) {
    let t19791 = t3227 * t19790;
    let t19792 = t1092 * t19791;
    let t19799 = t2825 * t6504;
    let t19800 = t1020 * t19799;
    let t19802 = t2822 * t6630;
    let t19804 = t9438 * t6625;
    let t19805 = t3200 * t19804;
    let t19807 = t13155 * t19396;
    (t19792, t19800, t19802, t19805, t19807)
}
