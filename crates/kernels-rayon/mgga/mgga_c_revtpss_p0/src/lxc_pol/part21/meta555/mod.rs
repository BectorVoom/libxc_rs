//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta555 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2242;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2243;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta555(t1284: f64, t5219: f64, t3624: f64, t1225: f64, t13312: f64, t1012: f64, t1230: f64, t5390: f64, t12879: f64, t1715: f64, t247: f64, t1261: f64, t16756: f64, t5341: f64, t3720: f64, t12916: f64, t5342: f64, t5340: f64, t12702: f64, t5330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17400, t17401, t17404, t17405, t17412, t17416, t17417) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2242(t1284, t5219, t3624, t1225, t13312, t1012, t1230, t5390, t12879, t1715, t247, t1261);
        let (t17419, t17420, t17423, t17425, t17426) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2243(t16756, t5341, t3720, t12916, t5342, t5340, t12702, t5330);
    (t17400, t17401, t17404, t17405, t17412, t17416, t17417, t17419, t17420, t17423, t17425, t17426)
}
