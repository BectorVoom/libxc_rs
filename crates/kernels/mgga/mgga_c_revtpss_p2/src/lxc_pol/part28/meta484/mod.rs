//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta484 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1837;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1838;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1839;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1840;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1841;
use chunk5::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1842;
use chunk6::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta484<F: Float>(t7135: F, t994: F, t11199: F, t1981: F, t7143: F, t1976: F, t3059: F, t7145: F, t1000: F, t1097: F, t1978: F, t25640: F, t25648: F, t25651: F, t25658: F, t25662: F, t25671: F, t25674: F, t25678: F, t25683: F, t25687: F, t25692: F, t3043: F, t3060: F, t3067: F, t3076: F, t3271: F, t3326: F, t342: F, t7102: F, t7137: F, t7140: F, t7144: F, t7156: F, t7167: F, t7170: F, t7174: F, t989: F, t25637: F, t3336: F, t7177: F, t11108: F, t1989: F, t14365: F, t1940: F, t1963: F, t198: F, t207: F, t2394: F, t2403: F, t2408: F, t2430: F, t25435: F, t25440: F, t25445: F, t2832: F, t4541: F, t7087: F, t7091: F, t775: F, t890: F, t892: F, t265: F, t393: F, t1100: F, t1102: F, t3329: F, t3333: F, t336: F, t5023: F, t7181: F, t30: F, t1996: F, t2258: F, t25459: F, t45: F, t606: F, t7194: F, t33: F, t2411: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t25695 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1837::<F>(t7135, t994);
        let t25698 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1838::<F>(t11199, t1981);
        let t25699 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1839::<F>(t25698, t7143);
        let (t25701, t25704) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1840::<F>(t1976, t3059, t7145, t1000, t1097, t1978, t25640, t25648, t25651, t25658, t25662, t25671, t25674, t25678, t25683, t25687, t25692, t25695, t25699, t3043, t3060, t3067, t3076, t3271, t3326, t342, t7102, t7137, t7140, t7144, t7156, t7167, t7170, t7174, t989);
        let (t25705, t25709, t25713, t25743) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1841::<F>(t25637, t25704, t3336, t7177, t11108, t1989, t14365, t1940, t1963, t198, t207, t2394, t2403, t2408, t2430, t25435, t25440, t25445, t2832, t4541, t7087, t7091, t775, t890, t892);
        let t25744 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1842::<F>(t265, t393, t1100, t1102, t198, t25705, t25709, t25713, t25743, t3329, t3333, t336, t5023, t7181);
        let (t25751, t25752, t25759) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1843::<F>(t30, t1996, t2258, t25459, t25744, t45, t606, t7194, t2394, t33, t2411, dens_threshold, rho0, zeta_threshold);
    (t25695, t25698, t25699, t25701, t25705, t25709, t25713, t25743, t25744, t25751, t25752, t25759)
}
