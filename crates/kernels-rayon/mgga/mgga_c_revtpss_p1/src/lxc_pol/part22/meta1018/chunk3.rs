//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3524/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3524(t11672: f64, t11883: f64, t15725: f64, t15830: f64, t16226: f64, t1675: f64, t19873: f64, t20083: f64, t42215: f64, t4831: f64, t54546: f64, t54550: f64, t54553: f64, t55356: f64, t6289: f64, t66128: f64, t66721: f64, t66731: f64, t66734: f64, t66739: f64, t66747: f64) -> f64 {
    let t66749 = -t66721 / 1296.0_f64 + 11.0_f64 / 324.0_f64 * t11883 * t6289 + 0.31758531939310916275e-3_f64 * t54546 + 0.3811023832717309953e-3_f64 * t54550 + 0.3811023832717309953e-3_f64 * t54553 + 0.30488190661738479624e-2_f64 * t11672 * t19873 - 0.3811023832717309953e-3_f64 * t66731 + 0.19055119163586549765e-2_f64 * t16226 * t66734 * t42215 * t66128 + 0.19055119163586549765e-3_f64 * t66739 + 0.85748036236139473944e-3_f64 * t15725 * t20083 - 0.30488190661738479624e-2_f64 * t55356 * t1675 - 0.30488190661738479624e-2_f64 * t15830 * t4831 + 0.3811023832717309953e-3_f64 * t66747;
    t66749
}
