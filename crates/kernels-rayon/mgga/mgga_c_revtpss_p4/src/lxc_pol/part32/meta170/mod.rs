//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk802;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk803;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk804;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk805;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta170(t2850: f64, t4574: f64, t128: f64, t1469: f64, t2857: f64, t606: f64, t904: f64, t4186: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4575, t4576) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk802(t2850, t4574, t128);
        let (t4578, t4579) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk803(t1469, t2857, t606);
        let (t4580, t4581) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk804(t4579, t904, t128);
        let t4583 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk805(t4186, t905);
        let (t4584, t4585) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk806(t4583, t904, t128);
    (t4575, t4576, t4578, t4579, t4580, t4581, t4583, t4584, t4585)
}
