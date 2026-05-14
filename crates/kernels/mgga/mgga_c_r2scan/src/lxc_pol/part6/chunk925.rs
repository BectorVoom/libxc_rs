//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 925/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk925<F: Float>(t552: F, t6566: F, t551: F, t2573: F, t6540: F, t360: F, t2551: F, t495: F, t2132: F, t2183: F) -> (F, F, F, F, F, F, F, F) {
    let t6567 = t552 * t6566;
    let t6568 = t551 * t6567;
    let t6571 = t6540 * t2573;
    let t6572 = t360 * t6571;
    let t6575 = t6540 * t2551;
    let t6576 = t360 * t6575;
    let t6579 = t6540 * t495;
    let t6580 = t360 * t6579;
    let t6583 = t2183 * t2132;
    (t6568, t6571, t6572, t6575, t6576, t6579, t6580, t6583)
}
