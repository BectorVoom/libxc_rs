//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta234 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk901;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk902;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta234<F: Float>(t1045: F, t373: F, t6299: F, t1042: F, t1668: F, t3155: F, t3162: F, t225: F, t6235: F, t366: F, t1066: F, t6100: F, t247: F, t3182: F, t6092: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6301, t6302, t6305) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk901::<F>(t1045, t373, t6299, t1042, t1668);
        let (t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6326) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk902::<F>(t373, t6305, t3155, t1042, t3162, t225, t6235, t366, t1066, t6100, t247, t3182, t6092);
    (t6301, t6302, t6305, t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6326)
}
