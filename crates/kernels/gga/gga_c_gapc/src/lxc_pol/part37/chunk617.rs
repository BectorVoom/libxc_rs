//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 617/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk617<F: Float>(t3806: F, t3807: F, t3810: F, t3829: F, t3830: F, t3831: F, t3834: F, t3904: F, t3910: F, t3914: F, t884: F, t125: F, t1458: F, t144: F, t667: F, t101: F, t1474: F) -> (F, F, F, F) {
    let t3916 = -t3914 * t884 + t3806 + t3807 - t3810 + t3829 - t3830 - t3831 + t3834 - t3904 + t3910;
    let t3938 = t1458 * t125;
    let t3940 = t667 * t144;
    let t3945 = t1474 * t101;
    (t3916, t3938, t3940, t3945)
}
