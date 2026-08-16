//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1777;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1778;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1779;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1780;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1781;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta498(t27937: f64, t27955: f64, t26016: f64, t26310: f64, t26312: f64, t26325: f64, t27933: f64, t27941: f64, t27943: f64, t27945: f64, t27947: f64, t27949: f64, t27951: f64, t27953: f64, t27957: f64, t28875: f64, t545: f64, t2028: f64, t689: f64, t8099: f64, t25904: f64, t25899: f64, t213: f64, t8085: f64, t1904: f64, t7492: f64, t225: f64, t27899: f64, t7515: f64, t2097: f64, t3999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t28887 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1777(t27937, t27955, t26016, t26310, t26312, t26325, t27933, t27941, t27943, t27945, t27947, t27949, t27951, t27953, t27957);
        let t28888 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1778(t28875, t28887);
        let (t28889, t28890, t28894) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1779(t28888, t545, t2028, t689, t8099);
        let (t28895, t28897, t28899, t28902, t28903, t28905, t28909) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1780(t25904, t28894, t25899, t213, t8085, t1904, t7492, t689, t225, t28888, t27899, t7515);
        let t28911 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1781(t2097, t3999);
    (t28888, t28889, t28890, t28894, t28895, t28897, t28899, t28902, t28903, t28905, t28909, t28911)
}
