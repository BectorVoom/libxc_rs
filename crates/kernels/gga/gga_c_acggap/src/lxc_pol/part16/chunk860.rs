//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 860/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk860<F: Float>(t29997: F, t7942: F, t8406: F, t525: F, t847: F, t7932: F, t7963: F, t2138: F, t2147: F, t322: F, t8436: F, t448: F, t8396: F, t315: F, t7966: F, t2137: F) -> (F, F, F, F, F, F, F) {
    let t33786 = 0.17347256376410398924e1 * t7942 * t29997 * t8406;
    let t33787 = t525 * t847;
    let t33789 = t7963 * t7932 * t33787;
    let t33794 = 0.34694512752820797848e1 * t2138 * t2147 * t8436 * t322;
    let t33795 = t8396 * t448;
    let t33796 = t315 * t33795;
    let t33798 = 0.17347256376410398924e1 * t33796 * t7966;
    let t33799 = t2137 * t33795;
    (t33786, t33789, t33794, t33795, t33796, t33798, t33799)
}
