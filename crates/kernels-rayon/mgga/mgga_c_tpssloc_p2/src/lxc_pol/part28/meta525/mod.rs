//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta525 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1775;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1776;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta525(t22828: f64, t80853: f64, t80855: f64, t22783: f64, t3872: f64, t1336: f64, t2690: f64, t6950: f64, t1369: f64, t22782: f64, t3777: f64, t3876: f64, t15: f64, t2229: f64, t1361: f64, t192: f64, t1995: f64, t22690: f64, t2230: f64, t22843: f64, t213: f64, t22847: f64, t22842: f64, t531: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80857, t80859, t80866, t80867, t80869, t80870, t80872) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1775(t22828, t80853, t80855, t22783, t3872, t1336, t2690, t6950, t1369, t22782, t3777, t3876);
        let (t80881, t80885, t80887, t80889, t80893) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1776(t15, t2229, t1361, t192, t1995, t22690, t2230, t22843, t213, t22847, t22842, t531, t598);
    (t80857, t80859, t80866, t80867, t80869, t80870, t80872, t80881, t80885, t80887, t80889, t80893)
}
