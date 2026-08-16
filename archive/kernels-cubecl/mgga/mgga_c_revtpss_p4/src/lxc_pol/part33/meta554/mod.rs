//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta554 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1942;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1943;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta554<F: Float>(t1949: F, t231: F, t6016: F, t7076: F, t1558: F, t1579: F, t25392: F, t5977: F, t2723: F, t25416: F, t1955: F, t6041: F, t1959: F, t25333: F, t25337: F, t25362: F, t25364: F, t25371: F, t25391: F, t25406: F, t25424: F, t27199: F, t27280: F, t27325: F, t27335: F, t27338: F, t27342: F, t27344: F, t7070: F, t7775: F, t29672: F, t892: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t29674, t29675, t29682, t29683, t29690, t29691, t29694, t29695, t29698) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1942::<F>(t1949, t231, t6016, t7076, t1558, t1579, t25392, t5977, t2723, t25416, t1955, t6041);
        let t29703 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1943::<F>(t1959, t25333, t25337, t25362, t25364, t25371, t25391, t25406, t25424, t27199, t27280, t27325, t27335, t27338, t27342, t27344, t29675, t29683, t29691, t29695, t29698, t7070, t7775);
        let (t29704, t29705) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1944::<F>(t29672, t29703, t892);
    (t29674, t29675, t29682, t29683, t29690, t29691, t29694, t29695, t29698, t29704, t29705)
}
