//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 693/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk693(t29055: f64, t4181: f64, t15460: f64, t10443: f64, t7032: f64, t1091: f64, t24908: f64, t2874: f64, t3746: f64, t6273: f64, t10261: f64, t309: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29056 = t29055 * t4181;
    let t29057 = t15460 * t29056;
    let t29060 = t10443 * t7032;
    let t29063 = t24908 * t1091;
    let t29064 = t2874 * t29063;
    let t29067 = t6273 * t3746;
    let t29068 = t2874 * t29067;
    let t29071 = t10261 * t309;
    (t29056, t29057, t29060, t29063, t29064, t29067, t29068, t29071)
}
