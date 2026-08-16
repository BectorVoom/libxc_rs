//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta139 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk726;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk727;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk728;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta139(t378: f64, t989: f64, t340: f64, t992: f64, t338: f64, t999: f64, t996: f64, t1071: f64, t994: f64, t1096: f64, t1079: f64, t2846: f64, t2848: f64, t2855: f64, t2860: f64, t2864: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3052, t3056, t3057) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk726(t378, t989, t340, t992, t338);
        let (t3058, t3059) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk727(t3057, t378, t999);
        let (t3060, t3063, t3066, t3067, t3070, t3075) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk728(t3059, t996, t1071, t994, t1096, t999, t1079, t2846, t2848, t2855, t2860, t2864);
    (t3052, t3056, t3057, t3058, t3059, t3060, t3063, t3066, t3067, t3070, t3075)
}
