//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3696/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3696(t17361: f64, t5293: f64, t1042: f64, t1252: f64, t17222: f64, t17505: f64, t17547: f64, t17796: f64, t1797: f64, t20809: f64, t3363: f64, t3368: f64, t3617: f64, t3711: f64, t3714: f64, t5274: f64, t5287: f64, t5384: f64, t59371: f64, t6573: f64, t69947: f64, t69958: f64, t69961: f64, t69964: f64, t69966: f64, t69968: f64) -> f64 {
    let t69971 = t5293 * t17361;
    let t69982 = -0.20325460441158986416e-2_f64 * t69947 + 0.28582678745379824648e-3_f64 * t3711 * t1042 * t20809 * t3368 - 0.22866142996303859718e-2_f64 * t59371 * t1797 - 0.45732285992607719436e-2_f64 * t17547 * t5287 + 0.42874018118069736972e-3_f64 * t69958 * t1252 - 0.33875767401931644026e-2_f64 * t69961 + 0.47637797908966374413e-4_f64 * t69964 + 0.3811023832717309953e-3_f64 * t69966 - 0.30488190661738479624e-2_f64 * t69968 * t3714 + 0.5081365110289746604e-3_f64 * t69971 + 0.47637797908966374413e-3_f64 * t5384 * t1042 * t3617 * t6573 * t3363 + 0.2540682555144873302e-2_f64 * t17505 * t17796 + 0.42874018118069736972e-3_f64 * t5274 * t17222;
    t69982
}
