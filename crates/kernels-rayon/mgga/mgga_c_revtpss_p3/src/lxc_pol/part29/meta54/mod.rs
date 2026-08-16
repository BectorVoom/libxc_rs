//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta54 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk343;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk344;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk345;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk346;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk347;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk348;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk349;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta54(t1032: f64, t342: f64, t358: f64, t360: f64, t336: f64, t368: f64, t365: f64, t246: f64, t372: f64, t912: f64, t938: f64, t978: f64, t980: f64, t985: f64, t373: f64, t357: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1033, t1034, t1035, t1036, t1038) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk343(t1032, t342, t358, t360, t336, t368);
        let t1040 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk344(t1038, t365, t1036);
        let t1041 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk345(t1033, t1040);
        let t1042 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk346(t246, t372);
        let t1043 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk347(t912, t938, t978, t980, t985);
        let (t1044, t1045) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk348(t1043, t373, t357, t73);
        let (t1046, t1047) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk349(t1044, t1045, t1042);
    (t1033, t1034, t1035, t1036, t1038, t1040, t1041, t1042, t1043, t1045, t1046, t1047)
}
