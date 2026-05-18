//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1104/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1104<F: Float>(t18679: F, t2763: F, t3699: F, t7730: F, t1899: F, t277: F, t33666: F, t26597: F, t2660: F, t16182: F, t102: F, t9281: F) -> (F, F, F, F) {
    let t33670 = t3699 * t18679 * t2763 * t7730;
    let t33671 = t277 * t1899 * t33666 * t33670;
    let t33673 = t2660 * t26597;
    let t33674 = t33673 * t16182;
    let t33676 = t9281 * t102;
    (t33671, t33673, t33674, t33676)
}
