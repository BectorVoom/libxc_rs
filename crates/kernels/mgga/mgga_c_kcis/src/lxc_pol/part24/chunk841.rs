//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 841/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk841<F: Float>(t5019: F, t5026: F, t1092: F, t13181: F, t1774: F, t2825: F, t6556: F, t113: F, t10443: F, t18443: F, t8: F, t1131: F, t1021: F, t13106: F, t1768: F, t13322: F, t4819: F) -> (F, F, F, F, F, F, F, F) {
    let t19644 = t5026 * t5019;
    let t19645 = t1092 * t19644;
    let t19647 = t13181 * t1774;
    let t19648 = t1092 * t19647;
    let t19650 = t2825 * t6556;
    let t19651 = t1092 * t19650;
    let t19653 = 2.0 * t113;
    let t19655 = t18443 * t8 + t10443 + t19653;
    let t19656 = t1131 * t19655;
    let t19657 = t1021 * t19656;
    let t19658 = t1092 * t19657;
    let t19660 = t13106 * t1768;
    let t19661 = t1092 * t19660;
    let t19663 = t13322 * t4819;
    (t19645, t19648, t19651, t19655, t19656, t19658, t19661, t19663)
}
