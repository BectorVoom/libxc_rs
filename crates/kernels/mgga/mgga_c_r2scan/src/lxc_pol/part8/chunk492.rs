//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 492/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk492<F: Float>(t183: F, t1883: F, t1800: F, t650: F, t653: F, t181: F, t648: F) -> (F, F, F) {
    let t1885 = 1.0 * t183 * t1883;
    let t1888 = 0.16081979498692535067e2 * t650 * t653 * t1800;
    let t1890 = 1.0 / t648 / t181;
    (t1885, t1888, t1890)
}
