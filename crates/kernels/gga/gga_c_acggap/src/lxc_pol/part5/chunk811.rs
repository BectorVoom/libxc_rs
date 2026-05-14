//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 811/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk811<F: Float>(t12457: F, t947: F, t3765: F, t932: F, t1077: F, t435: F, t3372: F, t3427: F, t1113: F, t3770: F, t1108: F, t1089: F, t175: F, t384: F, t839: F, t879: F) -> (F, F, F, F, F, F, F) {
    let t12458 = t12457 * t947;
    let t12460 = t3765 * t932;
    let t12473 = t435 * t1077;
    let t12478 = t3372 * t3427;
    let t12498 = t3770 * t1113;
    let t12511 = t3770 * t1108;
    let t12516 = t384 * t1089 * t175 * t839 * t879;
    (t12458, t12460, t12473, t12478, t12498, t12511, t12516)
}
