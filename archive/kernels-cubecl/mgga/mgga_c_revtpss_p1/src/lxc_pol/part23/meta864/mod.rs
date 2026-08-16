//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta864 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2756;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta864<F: Float>(t1444: F, t2782: F, t556: F, t6895: F, t9656: F, t22409: F, t2435: F, t13730: F, t1893: F, t3899: F, t689: F, t6919: F, t22449: F, t136: F, t2457: F, t6918: F, t9674: F, t13999: F, t22146: F, t22145: F, t48863: F, t49137: F, t124: F, t6861: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t73671, t73673, t73676, t73705) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2756::<F>(t1444, t2782, t556, t6895, t9656, t22409, t2435, t13730, t1893, t3899, t689, t6919);
        let (t73707, t73712, t73726, t73729, t73731) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2757::<F>(t22449, t2435, t136, t2457, t6918, t9674, t13999, t22146, t22145, t48863, t49137, t124, t6861);
    (t73671, t73673, t73676, t73705, t73707, t73712, t73726, t73729, t73731)
}
