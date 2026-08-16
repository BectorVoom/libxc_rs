//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta442 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1986;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta442(t1653: f64, t3509: f64, t3578: f64, t3516: f64, t1742: f64, t478: f64, t3068: f64, t1244: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t15559, t15560, t15563, t15564, t15567, t15568, t15569) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1986(t1653, t3509, t3578, t3516, t1742, t478, t3068, t1244);
    (t15559, t15560, t15563, t15564, t15567, t15568, t15569)
}
