//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2098/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2098<F: Float>(t1920: F, t2966: F, t6699: F, t1921: F, t82457: F, t23314: F, t23384: F, t6707: F, t82632: F, t23734: F, t3216: F, t11094: F, t6818: F) -> (F, F, F, F, F, F) {
    let t83444 = t1920 * t2966 * t6699;
    let t83453 = t1921 * t82457;
    let t83457 = t23384 * t23314;
    let t83459 = t82632 * t6707;
    let t83468 = t23734 * t3216;
    let t83472 = t6818 * t11094;
    (t83444, t83453, t83457, t83459, t83468, t83472)
}
