//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 919/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk919<F: Float>(t1850: F, t6667: F, t6934: F, t965: F, t6937: F, t16026: F, t1842: F, t16004: F, t1659: F, t11625: F, t1049: F, t4597: F, t11495: F, t11533: F, t11535: F, t11537: F, t11542: F, t11548: F, t11562: F, t11612: F, t11630: F, t15909: F, t15921: F, t15930: F, t165: F, t5089: F, t5168: F) -> (F,) {
    let t16246 = 0.47822877300252710492e-1 * t1850 * t6667;
    let t16251 = t965 * t6934;
    let t16254 = 0.17611111111111111111e-2 * t965 * t6937;
    let t16255 = t1842 * t16026;
    let t16258 = t1659 * t16004;
    let t16262 = 0.62154466893555682512e-3 * t11625 * t6667;
    let t16265 = t1049 * t4597;
    let t16269 = -0.21858666666666666666e-1 * t11533 + 0.70444444444444444443e-2 * t11535 - 0.9368e-2 * t11537 + 0.26416666666666666666e-2 * t11542 + 0.23526125e-4 * t11548 + 0.71734315950379065738e-1 * t11495 * t15930 - 0.62154466893555682512e-3 * t11630 * t15930 + t16246 - 0.23911438650126355246e-1 * t5089 * t15921 + 0.95645754600505420984e-1 * t11612 * t15909 + 0.52833333333333333333e-2 * t16251 + t16254 - 0.1585e-2 * t165 * t16255 + 0.317e-2 * t165 * t16258 - t16262 + 0.15538616723388920628e-3 * t5168 * t15921 - 0.62154466893555682512e-3 * t16265 * t15909 + 0.78420416666666666666e-4 * t11562;
    (t16269,)
}
