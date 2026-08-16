//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta484 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1837;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1838;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1839;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1840;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1841;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1842;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta484(t7135: f64, t994: f64, t11199: f64, t1981: f64, t7143: f64, t1976: f64, t3059: f64, t7145: f64, t1000: f64, t1097: f64, t1978: f64, t25640: f64, t25648: f64, t25651: f64, t25658: f64, t25662: f64, t25671: f64, t25674: f64, t25678: f64, t25683: f64, t25687: f64, t25692: f64, t3043: f64, t3060: f64, t3067: f64, t3076: f64, t3271: f64, t3326: f64, t342: f64, t7102: f64, t7137: f64, t7140: f64, t7144: f64, t7156: f64, t7167: f64, t7170: f64, t7174: f64, t989: f64, t25637: f64, t3336: f64, t7177: f64, t11108: f64, t1989: f64, t14365: f64, t1940: f64, t1963: f64, t198: f64, t207: f64, t2394: f64, t2403: f64, t2408: f64, t2430: f64, t25435: f64, t25440: f64, t25445: f64, t2832: f64, t4541: f64, t7087: f64, t7091: f64, t775: f64, t890: f64, t892: f64, t265: f64, t393: f64, t1100: f64, t1102: f64, t3329: f64, t3333: f64, t336: f64, t5023: f64, t7181: f64, t30: f64, t1996: f64, t2258: f64, t25459: f64, t45: f64, t606: f64, t7194: f64, t33: f64, t2411: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t25695 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1837(t7135, t994);
        let t25698 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1838(t11199, t1981);
        let t25699 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1839(t25698, t7143);
        let (t25701, t25704) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1840(t1976, t3059, t7145, t1000, t1097, t1978, t25640, t25648, t25651, t25658, t25662, t25671, t25674, t25678, t25683, t25687, t25692, t25695, t25699, t3043, t3060, t3067, t3076, t3271, t3326, t342, t7102, t7137, t7140, t7144, t7156, t7167, t7170, t7174, t989);
        let (t25705, t25709, t25713, t25743) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1841(t25637, t25704, t3336, t7177, t11108, t1989, t14365, t1940, t1963, t198, t207, t2394, t2403, t2408, t2430, t25435, t25440, t25445, t2832, t4541, t7087, t7091, t775, t890, t892);
        let t25744 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1842(t265, t393, t1100, t1102, t198, t25705, t25709, t25713, t25743, t3329, t3333, t336, t5023, t7181);
        let (t25751, t25752, t25759) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1843(t30, t1996, t2258, t25459, t25744, t45, t606, t7194, t2394, t33, t2411, dens_threshold, rho0, zeta_threshold);
    (t25695, t25698, t25699, t25701, t25705, t25709, t25713, t25743, t25744, t25751, t25752, t25759)
}
