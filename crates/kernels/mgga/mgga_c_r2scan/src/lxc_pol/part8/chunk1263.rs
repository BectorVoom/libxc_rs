//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1263/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1263<F: Float>(t3016: F, t551: F, t566: F, t6343: F, t3053: F, t6212: F, t6209: F, t6211: F, t24994: F, t8129: F, t783: F, t784: F, t788: F, t9083: F, t25363: F, t2687: F, t8081: F) -> (F, F, F, F, F) {
    let t29074 = t566 * t551 * t6343 * t3016;
    let t29126 = t6212 * t3053;
    let t29128 = t6209 * t6211 * t29126;
    let t29130 = t24994 * t8129;
    let t29146 = t783 * t9083 * t784 * t788;
    let t29152 = t25363 * t2687 * t8081;
    (t29074, t29128, t29130, t29146, t29152)
}
