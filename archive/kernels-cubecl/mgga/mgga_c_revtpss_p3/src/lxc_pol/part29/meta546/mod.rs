//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta546 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1883;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta546<F: Float>(t25878: F, t96239: F, t26230: F, t9670: F, t25895: F, t94633: F, t25899: F, t94639: F, t1358: F, t2439: F, t7506: F, t785: F) -> (F, F, F, F, F, F, F, F) {
        let (t96240, t96242, t96243, t96245, t96246, t96248, t96249, t96253) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1883::<F>(t25878, t96239, t26230, t9670, t25895, t94633, t25899, t94639, t1358, t2439, t7506, t785);
    (t96240, t96242, t96243, t96245, t96246, t96248, t96249, t96253)
}
