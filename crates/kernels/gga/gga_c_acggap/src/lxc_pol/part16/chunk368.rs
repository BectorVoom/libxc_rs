//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 368/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk368<F: Float>(t1083: F, t1782: F, t336: F, t1713: F, t337: F, t1124: F, t1126: F, t1130: F, t1474: F, t1481: F, t1766: F, t1770: F) -> (F, F, F) {
    let t1784 = t336 * t1083 * t1782;
    let t1788 = t336 * t337 * t1713;
    let t1795 = t1124 + 0.978e0 * t1474 - t1126 + 0.7335e0 * t1766 - 0.12225e0 * t1481 - 0.36675e0 * t1770 + t1130;
    (t1784, t1788, t1795)
}
