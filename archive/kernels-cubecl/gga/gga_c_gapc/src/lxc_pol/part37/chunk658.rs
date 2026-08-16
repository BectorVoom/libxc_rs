//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 658/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk658<F: Float>(t3806: F, t3807: F, t3810: F, t3829: F, t3830: F, t3831: F, t3834: F, t3904: F, t3910: F, t3914: F, t884: F, t125: F, t1458: F) -> (F, F) {
    let t3916 = -t3914 * t884 + t3806 + t3807 - t3810 + t3829 - t3830 - t3831 + t3834 - t3904 + t3910;
    let t3938 = t1458 * t125;
    (t3916, t3938)
}
