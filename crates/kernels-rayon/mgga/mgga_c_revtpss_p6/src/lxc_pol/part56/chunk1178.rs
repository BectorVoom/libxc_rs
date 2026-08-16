//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1178/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1178(t104527: f64, t33517: f64, t8937: f64, t1294: f64, t1469: f64, t1811: f64, t33468: f64, t33494: f64, t131395: f64, t26894: f64, t1042: f64, t105270: f64, t105460: f64, t1238: f64, t124619: f64, t124644: f64, t124646: f64, t124675: f64, t124814: f64, t124827: f64, t124928: f64, t124950: f64, t1252: f64, t31993: f64, t33425: f64, t33498: f64, t34918: f64, t34920: f64, t34961: f64, t3626: f64, t3719: f64, t5347: f64, t5405: f64) -> (f64, f64) {
    let t131483 = t8937 * t104527 * t33517;
    let t131497 = t1469 * t1294;
    let t131503 = t33468 * t1811 * t33494;
    let t131506 = t26894 * t131395;
    let t131512 = -0.5578099381357651623e-3_f64 * t124950 * t34920 + 0.16734298144072954869e-2_f64 * t124814 * t31993 * t3719 * t105270 - 0.3718732920905101082e-3_f64 * t131483 * t1252 - 0.17135921299530705785e1_f64 * t124928 * t34961 - 0.24791552806034007214e-3_f64 * t124619 - 0.11156198762715303246e-2_f64 * t124675 * t1042 * t34918 * t5405 + 0.16734298144072954869e-2_f64 * t124814 * t31993 * t3719 * t105460 + 0.18822977838986977999e-3_f64 * t33425 * t3626 * t124827 * t131497 + 0.5578099381357651623e-3_f64 * t131503 * t1238 + 0.29749863367240808656e-2_f64 * t131506 * t33498 - 0.56468933516960933998e-3_f64 * t124644 * t124646 * t5347;
    (t131497, t131512)
}
