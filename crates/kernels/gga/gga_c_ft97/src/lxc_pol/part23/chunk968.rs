//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 968/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk968<F: Float>(t1476: F, t2842: F, t4181: F, t15460: F, t10443: F, t7032: F, t1091: F, t24908: F, t2874: F, t3746: F, t6273: F, t10261: F, t309: F) -> (F, F, F, F, F, F, F, F, F) {
    let t29055 = t2842 * t1476;
    let t29056 = t29055 * t4181;
    let t29057 = t15460 * t29056;
    let t29060 = t10443 * t7032;
    let t29063 = t24908 * t1091;
    let t29064 = t2874 * t29063;
    let t29067 = t6273 * t3746;
    let t29068 = t2874 * t29067;
    let t29071 = t10261 * t309;
    (t29055, t29056, t29057, t29060, t29063, t29064, t29067, t29068, t29071)
}
