//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 740/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk740(t225: f64, t3552: f64, t480: f64, t371: f64, t482: f64, t676: f64, t481: f64, t1231: f64, t1256: f64, t1247: f64, t1261: f64, t1266: f64, t3591: f64, t3600: f64, t3606: f64, t3610: f64, t3613: f64, t3620: f64, t3625: f64, t3631: f64, t3637: f64, t3640: f64, t3644: f64, t3647: f64, t484: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3650 = t3552 * t225;
    let t3651 = t3650 * t480;
    let t3655 = t371 * t676 * t482;
    let t3657 = 0.47637797908966374413e-4_f64 * t481 * t3655;
    let t3658 = t1231 * t1256;
    let t3660 = 0.21437009059034868486e-3_f64 * t1247 * t3591 + 0.42874018118069736972e-3_f64 * t3600 * t3606 - 0.21437009059034868486e-3_f64 * t3610 * t3613 + 0.23818898954483187207e-3_f64 * t1261 * t3620 - 0.28582678745379824648e-3_f64 * t3625 * t3631 - 0.19055119163586549765e-3_f64 * t3637 - 0.14291339372689912324e-3_f64 * t1261 * t3640 - 0.28582678745379824648e-3_f64 * t1261 * t3644 - 0.28582678745379824648e-3_f64 * t3647 * t1266 + 0.21437009059034868486e-3_f64 * t3651 * t484 - t3657 + 0.28582678745379824648e-3_f64 * t3658;
    (t3650, t3651, t3655, t3657, t3658, t3660)
}
