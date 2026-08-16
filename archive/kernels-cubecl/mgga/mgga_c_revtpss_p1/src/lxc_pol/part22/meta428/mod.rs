//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta428 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2049;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta428<F: Float>(t1531: F, t37: F, t2612: F, t4392: F, t72: F, t757: F, t14425: F, t150: F, t190: F, t10608: F, t2258: F, t4402: F) -> (F, F, F, F, F, F, F, F) {
        let (t14613, t14615, t14616, t14618, t14619, t14620, t14621, t14622) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2049::<F>(t1531, t37, t2612, t4392, t72, t757, t14425, t150, t190, t10608, t2258, t4402);
    (t14613, t14615, t14616, t14618, t14619, t14620, t14621, t14622)
}
