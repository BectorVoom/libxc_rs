//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1204;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1205;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1206;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1207;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta308(t1211: f64, t12621: f64, t1207: f64, t456: f64, t487: f64, t1214: f64, t3568: f64, t1269: f64, t3566: f64, t1203: f64, t3565: f64, t3584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12622, t12625, t12626, t12627) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1204(t1211, t12621, t1207, t456);
        let (t12628, t12629) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1205(t12627, t487, t1214, t3568);
        let (t12630, t12633, t12640) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1206(t1211, t12629, t1269, t3566, t1203, t3565);
        let (t12641, t12646) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1207(t12640, t487, t1214, t3584);
    (t12622, t12625, t12626, t12627, t12628, t12629, t12630, t12633, t12640, t12641, t12646)
}
