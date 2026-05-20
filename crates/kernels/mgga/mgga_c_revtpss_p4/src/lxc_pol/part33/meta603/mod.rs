//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta603 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta603<F: Float>(t26894: F, t26921: F, t1294: F, t471: F, t355: F, t1210: F, t3627: F, t5457: F, t29193: F, t1203: F, t5464: F, t3566: F, t7627: F) -> (F, F, F, F, F, F, F, F) {
        let (t96927, t96929, t96953, t96954, t96979, t96982, t96986, t97019) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2027::<F>(t26894, t26921, t1294, t471, t355, t1210, t3627, t5457, t29193, t1203, t5464, t3566, t7627);
    (t96927, t96929, t96953, t96954, t96979, t96982, t96986, t97019)
}
