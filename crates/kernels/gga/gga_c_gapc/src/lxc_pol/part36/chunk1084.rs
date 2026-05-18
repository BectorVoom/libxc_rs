//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1084/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1084<F: Float>(t11987: F, t28609: F, t11399: F, t7877: F, t2554: F, t11977: F, t2153: F, t334: F, t3768: F, t3696: F, t3781: F, t11533: F, t761: F) -> (F, F, F, F, F, F) {
    let t33326 = t28609 * t11987;
    let t33328 = t11399 * t7877;
    let t33329 = t33328 * t2554;
    let t33330 = t11977 * t33329;
    let t33333 = t2153 * t3768 * t334;
    let t33336 = t2153 * t3696 * t3781;
    let t33338 = t761 * t11533;
    (t33326, t33328, t33330, t33333, t33336, t33338)
}
