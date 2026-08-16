//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3480/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3480(t1042: f64, t11714: f64, t1592: f64, t3127: f64, t42665: f64, t42672: f64, t4825: f64, t53290: f64, t53293: f64, t53926: f64, t54419: f64, t6308: f64, t6312: f64, t6331: f64, t65444: f64, t65446: f64, t65454: f64, t65456: f64, t65459: f64, t65462: f64) -> f64 {
    let t65468 = -0.28582678745379824648e-3_f64 * t3127 * t1042 * t54419 * t1592 + 0.3811023832717309953e-3_f64 * t65444 - 0.3811023832717309953e-3_f64 * t65446 + 0.42874018118069736972e-3_f64 * t42665 * t6308 - 0.21437009059034868486e-3_f64 * t42672 * t6312 + 0.30488190661738479624e-2_f64 * t11714 * t6331 - 0.3811023832717309953e-3_f64 * t65454 - 0.30488190661738479624e-2_f64 * t65456 - 0.19055119163586549766e-2_f64 * t65459 + 0.63517063878621832552e-3_f64 * t65462 + 0.30488190661738479624e-2_f64 * t53926 * t4825 - 0.30488190661738479624e-2_f64 * t53290 - 0.19055119163586549765e-3_f64 * t53293;
    t65468
}
