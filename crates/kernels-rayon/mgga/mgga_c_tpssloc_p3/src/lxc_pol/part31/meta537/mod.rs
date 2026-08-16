//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1752;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1753;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta537(t131: f64, t22791: f64, t9537: f64, t1338: f64, t225: f64, t236: f64, t1336: f64, t2690: f64, t6950: f64, t1369: f64, t22782: f64, t3777: f64, t15: f64, t2229: f64, t1361: f64, t192: f64, t1995: f64, t22690: f64, t2230: f64, t22843: f64, t213: f64, t22842: f64, t531: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t80853, t80854, t80855, t80866, t80867, t80869) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1752(t131, t22791, t9537, t1338, t225, t236, t1336, t2690, t6950, t1369, t22782, t3777);
        let (t80881, t80885, t80887, t80888, t80893) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1753(t15, t2229, t1361, t192, t1995, t22690, t2230, t22843, t213, t22842, t531, t598);
    (t80853, t80854, t80855, t80866, t80867, t80869, t80881, t80885, t80887, t80888, t80893)
}
