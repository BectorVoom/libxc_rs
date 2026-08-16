//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1226;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1227;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1228;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta192(t4579: f64, t904: f64, t128: f64, t4186: f64, t905: f64, t2847: f64, t2848: f64, t4571: f64, t4576: f64, t291: f64, t1596: f64, t914: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4580, t4581) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1226(t4579, t904, t128);
        let t4583 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1227(t4186, t905);
        let (t4584, t4585) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1228(t4583, t904, t128);
        let (t4587, t4589, t4590) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1229(t2847, t2848, t4571, t4576, t4581, t4585, t291, t1596, t914);
    (t4580, t4581, t4583, t4584, t4585, t4587, t4589, t4590)
}
