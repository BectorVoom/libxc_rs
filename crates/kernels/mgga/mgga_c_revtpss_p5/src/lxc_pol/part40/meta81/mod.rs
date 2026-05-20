//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta81 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk475;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk476;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk477;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk478;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta81<F: Float>(t1600: F, t916: F, t923: F, t1592: F, t930: F, t141: F, t1594: F, t921: F, t929: F, t935: F, t915: F, t939: F, t948: F, t951: F, t954: F, t958: F, t324: F, t967: F, t970: F, t973: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1601, t1604, t1606, t1607, t1609, t1610) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk475::<F>(t1600, t916, t923, t1592, t930, t141, t1594, t921, t929, t935);
        let (t1612, t1614, t1621, t1622) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk476::<F>(t1610, t915, t1594, t939, t1601, t1604, t1607, t948, t951, t954);
        let t1626 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk477::<F>(t1594, t958);
        let (t1627, t1633) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk478::<F>(t1626, t324, t1594, t1601, t1604, t1607, t967, t970);
        let t1634 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk479::<F>(t1633, t973);
    (t1606, t1609, t1610, t1612, t1614, t1621, t1622, t1626, t1627, t1633, t1634)
}
