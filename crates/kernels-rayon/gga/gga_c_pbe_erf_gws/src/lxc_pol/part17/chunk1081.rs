//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1081/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1081(t1192: f64, t2352: f64, t2409: f64, t3067: f64, t4007: f64, t6781: f64, t2417: f64, t9296: f64, t13772: f64, t13778: f64, t13785: f64, t13789: f64, t13794: f64, t13801: f64, t13804: f64, t13810: f64, t13813: f64, t13818: f64, t13822: f64, t13826: f64, t13833: f64, t13837: f64, t2408: f64, t3066: f64, t3207: f64, t335: f64, t827: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13840 = t1192 * t2352;
    let t13842 = t2409 * t3067 * t13840;
    let t13846 = t2409 * t6781 * t4007;
    let t13849 = t1192 * t2417;
    let t13851 = t2409 * t9296 * t13849;
    let t13854 = -t827 * t13772 / 48.0_f64 - t13778 / 192.0_f64 - t13785 / 768.0_f64 - t13789 / 3072.0_f64 - t13794 / 24.0_f64 + t13801 / 1536.0_f64 + t13804 / 1536.0_f64 - t13810 + t13813 / 96.0_f64 + t13818 / 96.0_f64 - t3207 * t13822 / 16.0_f64 - t335 * t13826 / 48.0_f64 + 5.0_f64 / 768.0_f64 * t13833 + t2408 * t13837 / 24.0_f64 + t3066 * t13842 / 48.0_f64 + t2408 * t13846 / 24.0_f64 - t3066 * t13851 / 16.0_f64;
    (t13840, t13842, t13846, t13849, t13851, t13854)
}
