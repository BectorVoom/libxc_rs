//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1909;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1910;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta510<F: Float>(t7769: F, t886: F, t25317: F, t225: F, t27265: F, t1579: F, t231: F, t836: F, t25392: F, t7048: F, t7071: F, t7759: F, t25399: F, t4481: F, t1580: F, t213: F, t25322: F, t25362: F, t25364: F, t25366: F, t25368: F, t25371: F, t25379: F, t25391: F, t257: F, t27199: F, t7070: F, t7079: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t27299, t27300, t27303, t27312, t27313, t27316, t27317, t27322) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1909::<F>(t7769, t886, t25317, t225, t27265, t1579, t231, t836, t25392, t7048, t7071, t7759);
        let t27329 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1910::<F>(t25399, t4481, t1580, t213, t25322, t25362, t25364, t25366, t25368, t25371, t25379, t25391, t257, t27199, t27300, t27303, t27313, t27317, t27322, t7070, t7079);
    (t27299, t27300, t27303, t27312, t27313, t27316, t27317, t27322, t27329)
}
