//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 700/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk700<F: Float>(t1360: F, t2317: F, t6525: F, t1365: F, t4325: F, t550: F, t6417: F, t158: F, t2293: F) -> (F, F, F, F, F) {
    let t6526 = t1360 * t2317;
    let t6527 = t6525 * t6526;
    let t6533 = t1365 * t4325;
    let t6534 = t6525 * t6533;
    let t6536 = t550 * t6417;
    let t6537 = t1365 * t6536;
    let t6540 = t158 * t2293;
    (t6527, t6534, t6536, t6537, t6540)
}
