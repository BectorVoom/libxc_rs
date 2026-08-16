//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 660/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk660<F: Float>(t231: F, t4088: F, t6045: F, t4125: F, t19116: F, t4093: F, t1701: F, t6027: F, t1196: F, t703: F, t684: F, t6035: F) -> (F, F, F, F, F, F) {
    let t28583 = t231 * t4088;
    let t28584 = t6045 * t28583;
    let t28587 = t231 * t4125;
    let t28591 = t19116 * t4093;
    let t28595 = t1701 * t6027 * t4088;
    let t28598 = t703 * t1196;
    let t28599 = t28598 * t684;
    let t28600 = t6035 * t28599;
    (t28584, t28587, t28591, t28595, t28599, t28600)
}
