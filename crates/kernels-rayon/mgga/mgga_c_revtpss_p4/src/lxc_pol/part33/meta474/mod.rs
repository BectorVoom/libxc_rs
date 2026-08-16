//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1724;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1725;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1726;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta474(t22252: f64, t543: f64, t1390: f64, t828: f64, t221: f64, t4019: f64, t6844: f64, t4018: f64, t14045: f64, t6869: f64, t3992: f64, t2661: f64, t6874: f64, t22079: f64, t5673: f64, t5675: f64, t1353: f64, t6836: f64, t9942: f64, t1868: f64, t5591: f64, t4012: f64, t1388: f64, t14013: f64, t14024: f64, t1410: f64, t22179: f64, t22183: f64, t5671: f64, t9953: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22253, t22255, t22259, t22260, t22262, t22264) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1724(t22252, t543, t1390, t828, t221, t4019, t6844, t4018, t14045, t6869, t3992, t2661);
        let (t22267, t22268, t22271, t22274, t22276, t22279) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1725(t221, t4019, t6874, t4018, t22079, t5673, t5675, t1353, t6836, t828, t9942, t1868, t5591);
        let (t22281, t22284) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1726(t22279, t4012, t828, t1388, t14013, t14024, t1410, t22179, t22183, t22255, t22260, t22264, t22268, t22271, t22276, t5671, t9953);
    (t22253, t22255, t22259, t22262, t22267, t22271, t22274, t22276, t22279, t22281, t22284)
}
