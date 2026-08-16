//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 767/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk767<F: Float>(t24: F, t9238: F, t2241: F, t645: F, t2307: F, t607: F, t65: F, t67: F, t1864: F, t2250: F, t2244: F, t628: F) -> (F, F, F, F, F, F) {
    let t9239 = t24 * t9238;
    let t9240 = t2241 * t645;
    let t9243 = t645 * t2307;
    let t9247 = t607 * t65 * t67;
    let t9248 = t1864 * t2250;
    let t9251 = t2244 * t628;
    (t9239, t9240, t9243, t9247, t9248, t9251)
}
