//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta536<F: Float>(t92997: F, t92999: F, t93007: F, t93012: F, t93020: F, t26482: F, t93321: F, t25375: F, t95628: F, t136: F, t137: F, t2061: F) -> (F, F, F, F, F, F, F, F) {
        let (t95674, t95675, t95678, t95680, t95684, t95720, t95722, t95725) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1868::<F>(t92997, t92999, t93007, t93012, t93020, t26482, t93321, t25375, t95628, t136, t137, t2061);
    (t95674, t95675, t95678, t95680, t95684, t95720, t95722, t95725)
}
