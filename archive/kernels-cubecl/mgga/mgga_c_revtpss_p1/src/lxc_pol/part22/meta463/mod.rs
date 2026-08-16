//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta463<F: Float>(t15547: F, t983: F, t3030: F, t4719: F, t3034: F, t11591: F, t1642: F, t11524: F, t4732: F, t981: F, t2989: F, t3336: F, t5019: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15549, t15551, t15553, t15555, t15556, t15558, t15559, t15561, t15562) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2144::<F>(t15547, t983, t3030, t4719, t3034, t11591, t1642, t11524, t4732, t981, t2989, t3336, t5019);
    (t15549, t15551, t15553, t15555, t15556, t15558, t15559, t15561, t15562)
}
