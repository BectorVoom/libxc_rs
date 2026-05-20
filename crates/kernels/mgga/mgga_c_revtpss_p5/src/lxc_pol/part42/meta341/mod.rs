//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1144;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta341<F: Float>(t15125: F, t15191: F, t4742: F, t993: F, t225: F, t366: F, t3224: F, t4845: F, t127: F, t371: F, t4852: F, t1025: F) -> (F, F, F, F, F, F, F) {
        let (t15638, t15639, t15654, t15655, t15656, t15662, t15668) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1144::<F>(t15125, t15191, t4742, t993, t225, t366, t3224, t4845, t127, t371, t4852, t1025);
    (t15638, t15639, t15654, t15655, t15656, t15662, t15668)
}
