//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 883/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk883(t13480: f64, t945: f64, t319: f64, t330: f64, t1072: f64, t4547: f64, t10096: f64, t2844: f64, t822: f64, t10087: f64, t10091: f64, t10093: f64, t10097: f64, t10099: f64, t10102: f64, t10104: f64, t10108: f64, t10109: f64, t10138: f64, t111: f64, t13132: f64, t13150: f64, t13173: f64, t2839: f64, t3061: f64, t3111: f64, t3150: f64, t317: f64, t323: f64, t333: f64, t8291: f64) -> f64 {
    let t13541 = t945 * t13480;
    let t13549 = t319 * t330;
    let t13558 = 0.47822877300252710492e-1_f64 * t1072 * t4547;
    let t13564 = 0.62154466893555682512e-3_f64 * t10096 * t4547;
    let t13567 = t822 * t2844;
    let t13576 = 0.317e-2_f64 * t111 * t13541 - 0.7026e-2_f64 * t317 * t3111 + 0.1585e-2_f64 * t323 * t8291 * t2839 + 0.10082625e-4_f64 * t333 * t10138 * t13549 + 0.71734315950379065738e-1_f64 * t10093 * t13132 - 0.62154466893555682512e-3_f64 * t10099 * t13132 + t13558 - 0.23911438650126355246e-1_f64 * t3061 * t13150 - 0.95645754600505420984e-1_f64 * t10108 * t13173 - t13564 + 0.15538616723388920628e-3_f64 * t3150 * t13150 + 0.62154466893555682512e-3_f64 * t13567 * t13173 + 0.10359077815592613752e-3_f64 * t10087 + 0.23911438650126355246e-1_f64 * t10091 - 0.31077233446777841256e-3_f64 * t10097 + 0.47822877300252710492e-1_f64 * t10102 - 0.11955719325063177623e-1_f64 * t10104 - 0.62154466893555682512e-3_f64 * t10109;
    t13576
}
