//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2101;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2102;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta643<F: Float>(t28056: F, t7732: F, t116: F, t29568: F, t5891: F, t94978: F, t665: F, t94982: F, t1513: F, t4287: F, t25826: F, t25823: F, t5915: F, t21876: F, t6998: F, t101454: F, t101456: F, t101754: F, t94974: F, t94976: F, t114: F, t508: F, t651: F, t28166: F, t7897: F, t28168: F, t22287: F, t28167: F, t8996: F, t5824: F, t775: F, t5966: F, t605: F) -> (F, F, F, F, F, F, F, F) {
        let (t105863, t105866, t105870, t105873, t105876, t105878) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2101::<F>(t28056, t7732, t116, t29568, t5891, t94978, t665, t94982, t1513, t4287, t25826, t25823, t5915);
        let t105885 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2102::<F>(t5915, t665, t25826, t21876, t6998, t101454, t101456, t101754, t105870, t105873, t105876, t105878, t94974, t94976);
        let (t105886, t105889, t105894, t105897, t105898, t105902) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2103::<F>(t114, t105885, t508, t651, t28166, t7897, t28168, t22287, t28167, t8996, t5824, t775, t5966, t605);
    (t105863, t105866, t105886, t105889, t105894, t105897, t105898, t105902)
}
