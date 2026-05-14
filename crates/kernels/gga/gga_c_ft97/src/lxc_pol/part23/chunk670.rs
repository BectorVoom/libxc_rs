//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 670/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk670<F: Float>(t3725: F, t6: F, t4952: F, t2393: F, t4947: F, t3771: F, t1109: F, t4951: F, t688: F, t4950: F, t1609: F, t236: F, t2378: F, t226: F, t3758: F) -> (F, F, F, F, F) {
    let t17820 = t3725 * t6;
    let t17821 = t17820 * t4952;
    let t17824 = t4947 * t2393;
    let t17825 = t3771 * t17824;
    let t17827 = t4951 * t1109 * t688;
    let t17828 = t4950 * t17827;
    let t17831 = t236 * t1609;
    let t17832 = t17831 * t2378;
    let t17833 = t3771 * t17832;
    let t17836 = t3758 * t226;
    (t17821, t17825, t17828, t17833, t17836)
}
