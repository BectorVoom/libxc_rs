//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3716/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3716(t21143: f64, t3636: f64, t17448: f64, t17580: f64, t17679: f64, t17684: f64, t17690: f64, t17732: f64, t21014: f64, t21017: f64, t21242: f64, t21267: f64, t3620: f64, t57128: f64, t57145: f64, t57164: f64, t57167: f64, t57170: f64, t57344: f64, t57707: f64, t70303: f64) -> f64 {
    let t70432 = t21143 * t3636;
    let t70453 = -0.19055119163586549765e-3_f64 * t70432 - 0.25724410870841842184e-2_f64 * t57344 * t21267 - 0.2540682555144873302e-2_f64 * t21242 * t3620 + 0.3811023832717309953e-3_f64 * t57128 + 0.3811023832717309953e-3_f64 * t57145 + 0.11433071498151929859e-2_f64 * t70303 * t17732 + 0.45732285992607719436e-2_f64 * t57707 * t17580 - 0.3811023832717309953e-3_f64 * t57164 - 0.3811023832717309953e-3_f64 * t57167 - 0.19055119163586549765e-3_f64 * t57170 + 0.47637797908966374413e-3_f64 * t17448 * t17690 + 0.30488190661738479624e-2_f64 * t21014 * t17679 - 0.15244095330869239812e-2_f64 * t21017 * t17684;
    t70453
}
