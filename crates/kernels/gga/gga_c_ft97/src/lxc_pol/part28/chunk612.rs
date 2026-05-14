//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 612/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk612<F: Float>(t16658: F, t2: F, t4: F, t26: F, t12277: F, t1384: F, t604: F, t6685: F) -> (F, F, F, F) {
    let t26579 = t16658 * t2;
    let t26580 = t26579 * t4;
    let t26581 = t26580 * t26;
    let t26584 = t12277 * t1384;
    let t26590 = t6685 * t604;
    (t26579, t26581, t26584, t26590)
}
