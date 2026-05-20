//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta405 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1482;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta405<F: Float>(t1453: F, t8406: F, t1843: F, t8320: F, t1310: F, t31027: F, t8395: F, t28036: F, t8311: F, t1513: F, t661: F, t8315: F) -> (F, F, F, F, F, F) {
        let (t31401, t31403, t31407, t31415, t31417, t31421) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1482::<F>(t1453, t8406, t1843, t8320, t1310, t31027, t8395, t28036, t8311, t1513, t661, t8315);
    (t31401, t31403, t31407, t31415, t31417, t31421)
}
