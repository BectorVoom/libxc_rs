//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 943/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk943<F: Float>(t2095: F, t31191: F, t2035: F, t420: F, t30059: F, t7544: F, t7676: F, t1095: F, t30572: F, t30573: F, t7458: F, t1988: F, t7689: F) -> (F, F, F, F, F, F) {
    let t31489 = t2095 * t31191;
    let t31491 = t2035 * t420;
    let t31492 = t31491 * t30059;
    let t31494 = t7676 * t7544;
    let t31495 = F::new(0.28303283060643736861e-2) * t31494;
    let t31498 = t30572 * t7458 * t1095 * t30573;
    let t31499 = F::new(0.62896184579208304135e-3) * t31498;
    let t31501 = t1988 * t7689;
    (t31489, t31491, t31492, t31495, t31499, t31501)
}
