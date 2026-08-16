//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta718 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2477;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta718<F: Float>(t48326: F, t47149: F, t3863: F, t5569: F, t3860: F, t5571: F, t9419: F, t1882: F, t4010: F, t2682: F, t4000: F, t5677: F, t820: F) -> (F, F, F, F, F, F, F) {
        let (t48327, t48330, t48332, t48334, t48335, t48455, t48486) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2477::<F>(t48326, t47149, t3863, t5569, t3860, t5571, t9419, t1882, t4010, t2682, t4000, t5677, t820);
    (t48327, t48330, t48332, t48334, t48335, t48455, t48486)
}
