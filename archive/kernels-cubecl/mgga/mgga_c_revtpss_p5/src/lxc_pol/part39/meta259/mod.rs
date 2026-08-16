//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta259 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk967;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk968;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk969;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk970;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta259<F: Float>(t8283: F, param_d: F, t116: F, t2178: F, t670: F, t117: F, t8273: F, t1459: F, t1461: F, t2187: F, t2189: F, t572: F, t573: F, t1843: F, t114: F, t1513: F, t8259: F, t1504: F, t8268: F, t8257: F, t8258: F, t8267: F) -> (F, F, F, F, F, F, F, F, F) {
        let t8289 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk967::<F>(t8283, param_d);
        let t8295 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk968::<F>(t116, t2178);
        let (t8296, t8299, t8302, t8353) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk969::<F>(t670, t8295, t117, t8273, t1459, t1461, t2187, t2189, t572, t573, t8289, t1843, t2178);
        let (t8355, t8358, t8362) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk970::<F>(t114, t1513, t8259, t1504, t8268, t8257, t8258, t8267);
    (t8289, t8295, t8296, t8299, t8302, t8353, t8355, t8358, t8362)
}
