//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2472/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2472<F: Float>(t3057: F, t4995: F, t3143: F, t42859: F, t342: F, t12032: F, t359: F, t3043: F, t3298: F, t16551: F, t994: F, t16558: F) -> (F, F, F, F, F, F, F) {
    let t43456 = t3057 * t4995;
    let t43471 = t42859 * t3143;
    let t43472 = t342 * t43471;
    let t43504 = t359 * t12032;
    let t43512 = t3043 * t3298;
    let t43520 = t994 * t16551;
    let t43524 = t994 * t16558;
    (t43456, t43471, t43472, t43504, t43512, t43520, t43524)
}
