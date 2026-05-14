//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 914/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk914<F: Float>(t1066: F, t154: F, t5688: F, t276: F, t2048: F, t2739: F, t7350: F, t742: F, t2932: F, t5974: F, t2104: F, t1885: F, t287: F, t1137: F, t5693: F, t1843: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7620 = t154 * t5688 * t1066;
    let t7621 = t276 * t7620;
    let t7628 = t154 * t2048 * t2739;
    let t7630 = t276 * t7628 / 144.0;
    let t7632 = t154 * t742 * t7350;
    let t7637 = t5974 * t2932;
    let t7639 = 0.57165357490759649296e-3 * t2104 * t7637;
    let t7640 = t287 * t1885;
    let t7641 = t1137 * t7640;
    let t7642 = t5693 * t7641;
    let t7648 = t287 * t1843;
    (t7620, t7621, t7628, t7630, t7632, t7637, t7639, t7640, t7641, t7642, t7648)
}
