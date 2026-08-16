//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1397;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta365<F: Float>(t14600: F, t676: F, t836: F, t14598: F, t1558: F, t879: F, t2482: F, t2801: F, t1531: F, t37: F, t4392: F, t72: F, t757: F, t1544: F, t2475: F, t124: F, t10779: F, t2749: F, t10777: F, t125: F, t4423: F, t136: F, t243: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14603, t14608, t14613, t14616) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1397::<F>(t14600, t676, t836, t14598, t1558, t879, t2482, t2801, t1531, t37, t4392, t72);
        let (t14618, t14648, t14671, t14673, t14675, t14676, t14685) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1398::<F>(t14616, t757, t1544, t2475, t124, t1558, t10779, t2749, t10777, t125, t4423, t136, t243);
    (t14603, t14608, t14613, t14618, t14648, t14671, t14673, t14675, t14676, t14685)
}
