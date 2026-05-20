//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta571 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta571<F: Float>(t14587: F, t689: F, t27312: F, t1568: F, t7063: F, t25410: F, t25304: F, t27212: F, t27349: F, t25260: F, t4368: F, t820: F, t844: F) -> (F, F, F, F, F, F, F) {
        let (t98809, t98815, t98848, t98849, t98867, t98892, t98937) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1918::<F>(t14587, t689, t27312, t1568, t7063, t25410, t25304, t27212, t27349, t25260, t4368, t820, t844);
    (t98809, t98815, t98848, t98849, t98867, t98892, t98937)
}
