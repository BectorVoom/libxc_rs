//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1724;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1725;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1726;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1727;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta482(t1032: f64, t1892: f64, t1955: f64, t1444: f64, t1883: f64, t7283: f64, t1426: f64, t7063: f64, t786: f64, t5629: f64, t7271: f64, t1885: f64, t26024: f64, t25972: f64, t5622: f64, t1889: f64, t25978: f64, t25986: f64, t5609: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27836, t27837) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1724(t1032, t1892, t1955);
        let (t27864, t27868) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1725(t1444, t1883, t1955, t7283);
        let t27883 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1726(t1426, t27836);
        let (t27884, t27899, t27919, t27921, t27924, t27926, t27928) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1727(t27883, t7063, t786, t5629, t7271, t1885, t26024, t25972, t5622, t1889, t25978, t25986, t5609);
    (t27836, t27837, t27864, t27868, t27883, t27884, t27899, t27919, t27921, t27924, t27926, t27928)
}
