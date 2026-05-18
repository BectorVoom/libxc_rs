//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 999/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk999<F: Float>(t1219: F, t615: F, t8396: F, t7987: F, t9054: F, t29997: F, t7942: F, t8406: F, t525: F, t847: F, t7932: F, t7963: F) -> (F, F, F, F) {
    let t33778 = t615 * t8396 * t1219;
    let t33783 = F::new(0.34694512752820797848e1) * t7987 * t9054;
    let t33786 = F::new(0.17347256376410398924e1) * t7942 * t29997 * t8406;
    let t33787 = t525 * t847;
    let t33789 = t7963 * t7932 * t33787;
    (t33778, t33783, t33786, t33789)
}
