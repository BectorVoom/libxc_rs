//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1280/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1280<F: Float>(t735: F, t9583: F, t154: F, t276: F, t3515: F, t5688: F, t300: F, t3650: F, t779: F, t2104: F, t5974: F, t9576: F, t9571: F, t5984: F, t9307: F, t17867: F, t3646: F) -> (F, F, F, F, F, F, F) {
    let t25212 = t735 * t9583;
    let t25218 = t276 * t154 * t5688 * t3515;
    let t25221 = t300 * t779 * t3650;
    let t25226 = t2104 * t5974 * t9576;
    let t25229 = t2104 * t5974 * t9571;
    let t25231 = t5984 * t9307;
    let t25236 = t2104 * t17867 * t3646;
    (t25212, t25218, t25221, t25226, t25229, t25231, t25236)
}
