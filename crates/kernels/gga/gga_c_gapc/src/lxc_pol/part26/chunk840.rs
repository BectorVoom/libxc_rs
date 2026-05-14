//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 840/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk840<F: Float>(t5708: F, t612: F, t144: F, t5713: F, t9066: F, t3060: F, t3687: F, t1040: F, t3065: F, t3688: F, t3071: F, t474: F, t1030: F, t3076: F, t11326: F, t3144: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11463 = t5708 * t612;
    let t11465 = t9066 * t144 * t5713;
    let t11466 = t11463 * t11465;
    let t11468 = t3060 * t3687;
    let t11469 = t11468 * t1040;
    let t11471 = t3688 * t3065;
    let t11473 = t474 * t3071;
    let t11474 = t1030 * t11473;
    let t11475 = t11474 * t3076;
    let t11477 = t11326 * t3144;
    (t11463, t11465, t11466, t11468, t11469, t11471, t11473, t11474, t11475, t11477)
}
