//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 815/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk815<F: Float>(t13480: F, t945: F, t319: F, t330: F, t1072: F, t4547: F, t10096: F, t2844: F, t822: F, t10087: F, t10091: F, t10093: F, t10097: F, t10099: F, t10102: F, t10104: F, t10108: F, t10109: F, t10138: F, t111: F, t13132: F, t13150: F, t13173: F, t2839: F, t3061: F, t3111: F, t3150: F, t317: F, t323: F, t333: F, t8291: F) -> (F,) {
    let t13541 = t945 * t13480;
    let t13549 = t319 * t330;
    let t13558 = 0.47822877300252710492e-1 * t1072 * t4547;
    let t13564 = 0.62154466893555682512e-3 * t10096 * t4547;
    let t13567 = t822 * t2844;
    let t13576 = 0.317e-2 * t111 * t13541 - 0.7026e-2 * t317 * t3111 + 0.1585e-2 * t323 * t8291 * t2839 + 0.10082625e-4 * t333 * t10138 * t13549 + 0.71734315950379065738e-1 * t10093 * t13132 - 0.62154466893555682512e-3 * t10099 * t13132 + t13558 - 0.23911438650126355246e-1 * t3061 * t13150 - 0.95645754600505420984e-1 * t10108 * t13173 - t13564 + 0.15538616723388920628e-3 * t3150 * t13150 + 0.62154466893555682512e-3 * t13567 * t13173 + 0.10359077815592613752e-3 * t10087 + 0.23911438650126355246e-1 * t10091 - 0.31077233446777841256e-3 * t10097 + 0.47822877300252710492e-1 * t10102 - 0.11955719325063177623e-1 * t10104 - 0.62154466893555682512e-3 * t10109;
    (t13576,)
}
