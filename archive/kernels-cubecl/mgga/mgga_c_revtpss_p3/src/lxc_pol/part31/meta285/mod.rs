//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1270;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta285<F: Float>(t3869: F, t9572: F, t2434: F, t762: F, t1331: F, t3860: F, t186: F, t685: F, t793: F, t1337: F, t4146: F, t565: F) -> (F, F, F, F, F, F, F) {
        let (t9574, t9575, t9577, t9578, t9586, t9588, t9593) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1270::<F>(t3869, t9572, t2434, t762, t1331, t3860, t186, t685, t793, t1337, t4146, t565);
    (t9574, t9575, t9577, t9578, t9586, t9588, t9593)
}
