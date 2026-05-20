//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta255 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk974;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk975;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk976;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk977;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta255<F: Float>(t8330: F, t116: F, t2198: F, param_d: F, t670: F, t117: F, t8320: F, t1459: F, t1461: F, t2207: F, t2209: F, t572: F, t573: F, t1843: F, t114: F, t1513: F, t8311: F, t109: F, t55: F, t655: F, t1509: F, t8315: F, t69: F, t8258: F, t8267: F, t8310: F, t508: F, t569: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t8336, t8342) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk974::<F>(t8330, t116, t2198, param_d);
        let (t8343, t8346, t8349, t8393) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk975::<F>(t670, t8342, t117, t8320, t1459, t1461, t2207, t2209, t572, t573, t8336, t1843, t2198);
        let (t8395, t8399, t8402, t8406) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk976::<F>(t114, t1513, t8311, t109, t55, t655, t1509, t8315, t69, t8258, t8267, t8310);
        let (t8407, t8411) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk977::<F>(t508, t8406, t569);
    (t8336, t8342, t8343, t8346, t8349, t8393, t8395, t8399, t8402, t8406, t8407, t8411)
}
