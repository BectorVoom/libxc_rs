//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1288;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta304<F: Float>(t225: F, t9801: F, t4062: F, t3889: F, t543: F, t1386: F, t2482: F, t814: F, t136: F, t1412: F, t220: F, t124: F, t1398: F, t3938: F, t4003: F, t4056: F, t2735: F, t4086: F, t3994: F, t808: F, t521: F, t9342: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9802, t9804, t9810, t9816, t9817, t9818, t9819) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1288::<F>(t225, t9801, t4062, t3889, t543, t1386, t2482, t814, t136, t1412, t220, t124, t1398);
        let (t9821, t9822, t9840, t9845, t9847, t9854) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1289::<F>(t3938, t9818, t9819, t9816, t4003, t4056, t2735, t4086, t3994, t808, t521, t9342);
    (t9802, t9804, t9810, t9816, t9817, t9818, t9821, t9822, t9840, t9845, t9847, t9854)
}
