//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta486 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta486<F: Float>(t25826: F, t28036: F, t4287: F, t6998: F, t4237: F, t76: F, t13269: F, t38: F, t1497: F, t640: F, t77: F, t4241: F, t84: F) -> (F, F, F, F, F, F, F) {
        let (t28037, t28039, t28089, t28093, t28104, t28105, t28108) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1769::<F>(t25826, t28036, t4287, t6998, t4237, t76, t13269, t38, t1497, t640, t77, t4241, t84);
    (t28037, t28039, t28089, t28093, t28104, t28105, t28108)
}
