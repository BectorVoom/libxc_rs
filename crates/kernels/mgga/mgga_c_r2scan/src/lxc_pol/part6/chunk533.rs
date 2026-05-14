//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 533/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk533<F: Float>(t1898: F, t621: F, t650: F, t1800: F, t190: F, t632: F, t175: F, t648: F) -> (F, F, F) {
    let t1901 = 0.32163958997385070134e2 * t650 * t1898 * t621;
    let t1904 = 2.0 * t632 * t190 * t1800;
    let t1906 = 1.0 / t648 / t175;
    (t1901, t1904, t1906)
}
