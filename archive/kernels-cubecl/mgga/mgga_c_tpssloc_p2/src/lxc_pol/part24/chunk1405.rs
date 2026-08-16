//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1405/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1405<F: Float>(t23587: F, t6698: F, t3166: F, t6688: F, t23384: F, t23399: F, t6692: F, t82573: F, t1920: F, t2966: F, t6699: F, t1921: F, t82457: F) -> (F, F, F, F, F, F) {
    let t83420 = t6698 * t23587;
    let t83424 = t6688 * t3166;
    let t83435 = t23384 * t23399;
    let t83441 = t82573 * t6692;
    let t83444 = t1920 * t2966 * t6699;
    let t83453 = t1921 * t82457;
    (t83420, t83424, t83435, t83441, t83444, t83453)
}
