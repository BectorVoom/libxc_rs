//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta520<F: Float>(t4975: F, t988: F, t27651: F, t4976: F, t27418: F, t994: F, t1096: F, t27638: F, t3143: F, t1983: F, t27642: F, t4983: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t27652, t27653, t27656, t27661, t27664, t27665, t27668, t27669, t27670) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1942::<F>(t4975, t988, t27651, t4976, t27418, t994, t1096, t27638, t3143, t1983, t27642, t4983);
    (t27652, t27653, t27656, t27661, t27664, t27665, t27668, t27669, t27670)
}
