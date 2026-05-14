//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 997/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk997<F: Float>(t1485: F, t1499: F, t1531: F, t126: F, t82: F, t94: F, t98: F, t501: F, t5175: F, t5075: F, t512: F, t83: F, t1511: F, t5336: F, t204: F, t99: F) -> (F, F, F, F, F, F) {
    let t16889 = 0.43374325201206959368e-1 * t1531 * t1485 * t1499;
    let t16893 = 24.0 * t82 * t94 * t98 * t126;
    let t16894 = t501 * t5175;
    let t16897 = t83 * t512 * t5075;
    let t16901 = t1511 * t5336;
    let t16903 = t99 * t204;
    (t16889, t16893, t16894, t16897, t16901, t16903)
}
