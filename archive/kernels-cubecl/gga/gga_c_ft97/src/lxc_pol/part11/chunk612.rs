//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 612/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk612<F: Float>(t494: F, t8232: F, t1882: F, t1897: F, t1588: F, t1871: F, t499: F, t1893: F, t454: F, t1855: F, t1580: F, t492: F) -> (F, F, F, F, F, F, F) {
    let t8475 = t8232 * t494;
    let t8477 = t1882 * t1897;
    let t8480 = t1871 * t499 * t1588;
    let t8483 = t1882 * t1893;
    let t8485 = t8232 * t454;
    let t8487 = t1882 * t1855;
    let t8489 = t1580 * t492;
    (t8475, t8477, t8480, t8483, t8485, t8487, t8489)
}
