//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 662/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk662<F: Float>(t2612: F, t551: F, t552: F, t1600: F, t928: F, t1632: F, t921: F, t574: F, t481: F, t910: F) -> (F, F, F, F, F) {
    let t2614 = t551 * t552 * t2612;
    let t2617 = t1600 * t928;
    let t2620 = t551 * t1632 * t921;
    let t2621 = t574 * t2620;
    let t2625 = t910 * t481;
    (t2614, t2617, t2620, t2621, t2625)
}
