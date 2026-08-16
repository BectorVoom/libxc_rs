//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta184 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk916;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk917;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk918;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk919;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta184(t235: f64, t3999: f64, t239: f64, t820: f64, t543: f64, t3923: f64, t1390: f64, t828: f64, t531: f64, t549: f64, t240: f64, t72: f64, t3829: f64, t1386: f64, t2482: f64, t27: f64, t136: f64, t1389: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t4000 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk916(t235, t3999);
        let (t4002, t4003) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk917(t239, t4000, t820, t543);
        let t4004 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk918(t3923, t4003);
        let (t4006, t4010, t4011, t4012, t4014, t4018) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk919(t1390, t4004, t828, t531, t549, t240, t72, t3829, t1386, t2482, t27);
        let t4019 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk920(t136, t1389);
    (t4000, t4002, t4003, t4004, t4006, t4010, t4011, t4012, t4014, t4018, t4019)
}
