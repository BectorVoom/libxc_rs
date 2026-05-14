//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 844/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk844<F: Float>(t1020: F, t19691: F, t1662: F, t4818: F, t14072: F, t3200: F, t4823: F, t9517: F, t6491: F, t922: F, t6704: F, t3210: F, t1773: F, t829: F, t4566: F, t13410: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19692 = t1020 * t19691;
    let t19694 = t1662 * t4818;
    let t19695 = t14072 * t19694;
    let t19696 = t3200 * t19695;
    let t19698 = t1662 * t4823;
    let t19699 = t9517 * t19698;
    let t19700 = t3200 * t19699;
    let t19702 = t6491 * t922;
    let t19703 = t9517 * t19702;
    let t19704 = t3200 * t19703;
    let t19706 = t6704 * t922;
    let t19707 = t3210 * t19706;
    let t19708 = t3200 * t19707;
    let t19710 = t1773 * t829;
    let t19711 = t4566 * t19710;
    let t19712 = t13410 * t19711;
    (t19692, t19694, t19696, t19698, t19700, t19702, t19704, t19706, t19708, t19710, t19711, t19712)
}
