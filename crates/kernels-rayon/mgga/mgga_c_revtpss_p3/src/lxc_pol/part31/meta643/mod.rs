//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2101;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2102;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta643(t28056: f64, t7732: f64, t116: f64, t29568: f64, t5891: f64, t94978: f64, t665: f64, t94982: f64, t1513: f64, t4287: f64, t25826: f64, t25823: f64, t5915: f64, t21876: f64, t6998: f64, t101454: f64, t101456: f64, t101754: f64, t94974: f64, t94976: f64, t114: f64, t508: f64, t651: f64, t28166: f64, t7897: f64, t28168: f64, t22287: f64, t28167: f64, t8996: f64, t5824: f64, t775: f64, t5966: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t105863, t105866, t105870, t105873, t105876, t105878) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2101(t28056, t7732, t116, t29568, t5891, t94978, t665, t94982, t1513, t4287, t25826, t25823, t5915);
        let t105885 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2102(t5915, t665, t25826, t21876, t6998, t101454, t101456, t101754, t105870, t105873, t105876, t105878, t94974, t94976);
        let (t105886, t105889, t105894, t105897, t105898, t105902) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2103(t114, t105885, t508, t651, t28166, t7897, t28168, t22287, t28167, t8996, t5824, t775, t5966, t605);
    (t105863, t105866, t105886, t105889, t105894, t105897, t105898, t105902)
}
