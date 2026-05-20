//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk971;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk972;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk973;
use chunk3::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk974;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta254<F: Float>(t1453: F, t2178: F, t1312: F, t2179: F, t2181: F, t2322: F, t4254: F, t5523: F, t651: F, t8254: F, t8274: F, t8278: F, t3: F, t116: F, param_d: F, t670: F, t117: F, t8273: F, t1459: F, t1461: F, t2187: F, t2189: F, t572: F, t573: F, t1843: F, t114: F, t1513: F, t8259: F, t1504: F, t8268: F, t8257: F, t8258: F, t8267: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t8280, t8283) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk971::<F>(t1453, t2178, t1312, t2179, t2181, t2322, t4254, t5523, t651, t8254, t8274, t8278);
        let (t8284, t8289, t8295) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk972::<F>(t3, t8283, t116, t2178, param_d);
        let (t8296, t8299, t8302, t8353) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk973::<F>(t670, t8295, t117, t8273, t1459, t1461, t2187, t2189, t572, t573, t8289, t1843, t2178);
        let (t8355, t8358, t8362) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk974::<F>(t114, t1513, t8259, t1504, t8268, t8257, t8258, t8267);
    (t8280, t8283, t8284, t8289, t8295, t8296, t8299, t8302, t8353, t8355, t8358, t8362)
}
