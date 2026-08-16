//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta540 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta540<F: Float>(t29543: F, t77: F, t5872: F, t84: F, t5819: F, t603: F, t5826: F, t5816: F, t1923: F, t1928: F, t25157: F, t28127: F, t28138: F, t28151: F, t28154: F, t29513: F, t29526: F, t29529: F, t29533: F, t29538: F, t6958: F, t7702: F, t7706: F, t7709: F, t7716: F, t7720: F) -> (F, F, F, F, F, F, F, F) {
        let (t29544, t29547, t29548, t29551, t29554, t29561, t29562, t29567) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1925::<F>(t29543, t77, t5872, t84, t5819, t603, t5826, t5816, t1923, t1928, t25157, t28127, t28138, t28151, t28154, t29513, t29526, t29529, t29533, t29538, t6958, t7702, t7706, t7709, t7716, t7720);
    (t29544, t29547, t29548, t29551, t29554, t29561, t29562, t29567)
}
