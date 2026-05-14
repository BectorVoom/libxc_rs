//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1001/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1001<F: Float>(t11625: F, t7715: F, t8626: F, t965: F, t8629: F, t970: F, t11495: F, t11528: F, t11532: F, t11533: F, t11535: F, t11562: F, t11612: F, t11630: F, t16246: F, t16251: F, t16254: F, t16262: F, t16265: F, t22289: F, t22294: F, t23220: F, t5089: F, t5168: F) -> (F,) {
    let t23225 = t11625 * t7715;
    let t23229 = t965 * t8626;
    let t23231 = t970 * t8629;
    let t23233 = t11528 + t11532 - 0.10929333333333333333e-1 * t11533 + 0.35222222222222222222e-2 * t11535 + t16246 - 0.62154466893555682512e-3 * t11630 * t22289 - 0.62154466893555682512e-3 * t16265 * t22294 + 0.71734315950379065738e-1 * t11495 * t22289 + 0.95645754600505420984e-1 * t11612 * t22294 - 0.23911438650126355246e-1 * t5089 * t23220 + 0.15538616723388920628e-3 * t5168 * t23220 - 0.31077233446777841256e-3 * t23225 + 0.52833333333333333332e-2 * t16251 + t16254 - t16262 + 0.39210208333333333333e-4 * t11562 + 0.88055555555555555555e-3 * t23229 - 0.117630625e-4 * t23231;
    (t23233,)
}
