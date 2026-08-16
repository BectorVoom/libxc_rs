//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1771;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta402(t1774: f64, t3617: f64, t372: f64, t5268: f64, t473: f64, t5412: f64, t13147: f64, t487: f64, t460: f64, t12050: f64, t13045: f64, t13141: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t17794, t17799, t17821, t17845, t17846, t17847, t17852) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1771(t1774, t3617, t372, t5268, t473, t5412, t13147, t487, t460, t12050, t13045, t13141);
    (t17794, t17799, t17821, t17845, t17846, t17847, t17852)
}
