//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1948;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta609<F: Float>(t106625: F, t25207: F, t27375: F, t63185: F, t11064: F, t1544: F, t27384: F, t25759: F, t77425: F, t100987: F, t29598: F, t94245: F) -> (F, F, F, F, F, F, F) {
        let (t106626, t107793, t107805, t107882, t107885, t107892, t107895) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1948::<F>(t106625, t25207, t27375, t63185, t11064, t1544, t27384, t25759, t77425, t100987, t29598, t94245);
    (t106626, t107793, t107805, t107882, t107885, t107892, t107895)
}
