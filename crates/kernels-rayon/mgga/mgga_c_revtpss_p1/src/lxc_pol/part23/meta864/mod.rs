//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta864 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2756;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta864(t1444: f64, t2782: f64, t556: f64, t6895: f64, t9656: f64, t22409: f64, t2435: f64, t13730: f64, t1893: f64, t3899: f64, t689: f64, t6919: f64, t22449: f64, t136: f64, t2457: f64, t6918: f64, t9674: f64, t13999: f64, t22146: f64, t22145: f64, t48863: f64, t49137: f64, t124: f64, t6861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t73671, t73673, t73676, t73705) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2756(t1444, t2782, t556, t6895, t9656, t22409, t2435, t13730, t1893, t3899, t689, t6919);
        let (t73707, t73712, t73726, t73729, t73731) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2757(t22449, t2435, t136, t2457, t6918, t9674, t13999, t22146, t22145, t48863, t49137, t124, t6861);
    (t73671, t73673, t73676, t73705, t73707, t73712, t73726, t73729, t73731)
}
