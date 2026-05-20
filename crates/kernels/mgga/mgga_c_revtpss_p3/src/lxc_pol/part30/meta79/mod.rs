//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta79 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk510;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk511;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta79<F: Float>(t1469: F, t905: F, t904: F, t128: F, t903: F, t291: F, t902: F, t916: F, t923: F, t930: F, t141: F, t921: F, t929: F, t935: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1592 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk510::<F>(t1469, t905);
        let (t1593, t1594, t1596, t1598, t1600) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk511::<F>(t1592, t904, t128, t903, t291, t902);
        let (t1601, t1604, t1606, t1607, t1609, t1610) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk512::<F>(t1600, t916, t923, t1592, t930, t141, t1594, t921, t929, t935);
    (t1592, t1593, t1594, t1596, t1598, t1600, t1601, t1604, t1606, t1607, t1609, t1610)
}
