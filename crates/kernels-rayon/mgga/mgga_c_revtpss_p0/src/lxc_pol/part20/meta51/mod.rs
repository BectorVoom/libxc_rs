//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta51 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk346;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk347;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk348;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk349;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk350;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk351;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk352;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta51(t196: f64, t342: f64, t358: f64, t360: f64, t336: f64, t368: f64, t365: f64, t246: f64, t372: f64, t912: f64, t938: f64, t978: f64, t980: f64, t985: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1031, t1032) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk346(t196);
        let (t1033, t1034, t1035) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk347(t1032, t342, t358);
        let t1036 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk348(t1035, t360);
        let t1038 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk349(t336, t368);
        let (t1040, t1041) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk350(t1038, t365, t1036, t1033);
        let t1042 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk351(t246, t372);
        let t1043 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk352(t912, t938, t978, t980, t985);
    (t1031, t1032, t1033, t1034, t1035, t1036, t1038, t1040, t1041, t1042, t1043)
}
