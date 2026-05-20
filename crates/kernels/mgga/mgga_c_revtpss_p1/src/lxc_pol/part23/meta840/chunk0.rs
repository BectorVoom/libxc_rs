//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2714/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2714<F: Float>(t17217: F, t17505: F, t1032: F, t1246: F, t21333: F, t17720: F, t5391: F, t11262: F, t3610: F, t6634: F, t17569: F, t5326: F, t5390: F) -> (F, F, F, F, F, F) {
    let t69947 = t17505 * t17217;
    let t69958 = t21333 * t1032 * t1246;
    let t69961 = t5391 * t17720;
    let t69964 = t3610 * t11262 * t6634;
    let t69966 = t17569 * t17217;
    let t69968 = t5326 * t5390;
    (t69947, t69958, t69961, t69964, t69966, t69968)
}
