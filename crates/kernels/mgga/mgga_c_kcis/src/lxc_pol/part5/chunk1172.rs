//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1172/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1172<F: Float>(t19688: F, t4994: F, t13181: F, t1713: F, t1020: F, t1662: F, t4818: F, t14072: F, t3200: F, t4823: F, t9517: F, t6491: F, t922: F) -> (F, F, F, F, F) {
    let t19689 = t4994 * t19688;
    let t19691 = t13181 * t1713;
    let t19692 = t1020 * t19691;
    let t19694 = t1662 * t4818;
    let t19695 = t14072 * t19694;
    let t19696 = t3200 * t19695;
    let t19698 = t1662 * t4823;
    let t19699 = t9517 * t19698;
    let t19700 = t3200 * t19699;
    let t19702 = t6491 * t922;
    (t19689, t19692, t19696, t19700, t19702)
}
