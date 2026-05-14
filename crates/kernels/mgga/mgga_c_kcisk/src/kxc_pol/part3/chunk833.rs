//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 833/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk833<F: Float>(t1253: F, t4075: F, t4083: F, t1229: F, t4030: F, t1254: F, t13589: F, t1255: F, t370: F, t4125: F, t13562: F, t4129: F, t13526: F, t13530: F, t13546: F, t13552: F, t13595: F, t13598: F, t13601: F, t13605: F, t13609: F, t13612: F, t13616: F, t13630: F, t13634: F, t13636: F) -> (F, F, F, F, F, F) {
    let t13702 = t4075 * t4083 * t1253;
    let t13705 = t1229 * t4030;
    let t13708 = t13589 * t1254;
    let t13711 = t1255 * t4075;
    let t13715 = 1.0 / t4125 / t370;
    let t13717 = t13715 * t13562 * t4129;
    let t13734 = -0.66228e0 * t13595 + 0.33114e0 * t13598 - 0.99342e0 * t13601 + 0.11038e0 * t13605 - 0.73586666666666666666e-1 * t13609 - 0.16557e0 * t13612 - 0.5519e0 * t13616 + 0.258925e1 * t13630 - 0.412621875e-1 * t13634 + 0.16504875e0 * t13636 - 0.60384999999999999999e0 * t13546 + 0.181155e1 * t13552 - 0.40256666666666666668e0 * t13526 + 0.20128333333333333333e0 * t13530;
    (t13702, t13705, t13708, t13711, t13717, t13734)
}
