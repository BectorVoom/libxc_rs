//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk726;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk727;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta139<F: Float>(t378: F, t989: F, t340: F, t992: F, t338: F, t999: F, t996: F, t1071: F, t994: F, t1096: F, t1079: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3052, t3056, t3057) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk726::<F>(t378, t989, t340, t992, t338);
        let (t3058, t3059) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk727::<F>(t3057, t378, t999);
        let (t3060, t3063, t3066, t3067, t3070, t3075) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk728::<F>(t3059, t996, t1071, t994, t1096, t999, t1079, t2846, t2848, t2855, t2860, t2864);
    (t3052, t3056, t3057, t3058, t3059, t3060, t3063, t3066, t3067, t3070, t3075)
}
