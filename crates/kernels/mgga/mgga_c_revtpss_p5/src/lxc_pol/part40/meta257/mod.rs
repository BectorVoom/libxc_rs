//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta257 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk957;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk958;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk959;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk960;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk961;
use chunk5::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk962;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta257<F: Float>(t3: F, t5789: F, t116: F, t1518: F, t670: F, t117: F, t4292: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, param_d: F, t159: F, t793: F, t94: F, t93: F, t2339: F, t69: F, t655: F, t1310: F, t2198: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5790, t5795, t5801, t5802, t5805, t5808) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk957::<F>(t3, t5789, t116, t1518, t670, t117, t4292, t1459, t1461, t1916, t1918, t572, t573, param_d);
        let (t7021, t7732) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk958::<F>(t159, t793, t1518, t94);
        let t7889 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk959::<F>(t1518, t93);
        let t8258 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk960::<F>(t2339, t69);
        let t8267 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk961::<F>(t655, t69);
        let t8307 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk962::<F>(t1310, t2198);
    (t5790, t5795, t5801, t5802, t5805, t5808, t7021, t7732, t7889, t8258, t8267, t8307)
}
