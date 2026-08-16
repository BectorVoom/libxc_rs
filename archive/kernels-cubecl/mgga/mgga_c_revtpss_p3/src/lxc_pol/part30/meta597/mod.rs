//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2058;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta597<F: Float>(t26884: F, t460: F, t1210: F, t26921: F, t3627: F, t5457: F, t26983: F, t7635: F, t29193: F, t26894: F, t3566: F, t7627: F) -> (F, F, F, F, F, F, F) {
        let (t96938, t96953, t96954, t96966, t96979, t96986, t97019) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2058::<F>(t26884, t460, t1210, t26921, t3627, t5457, t26983, t7635, t29193, t26894, t3566, t7627);
    (t96938, t96953, t96954, t96966, t96979, t96986, t97019)
}
