//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 622/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk622<F: Float>(t11288: F, t921: F, t1016: F, t10283: F, t3366: F, t8045: F, t2798: F, t3418: F, t3553: F, t6556: F, t4349: F, t1382: F, t2355: F, t3599: F, t11402: F, t895: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13334 = t11288 * t921;
    let t13336 = 2.0 * t10283 * t1016;
    let t13338 = 4.0 * t8045 * t3366;
    let t13340 = 2.0 * t2798 * t3418;
    let t13342 = 2.0 * t6556 * t3553;
    let t13343 = t3553 * t921;
    let t13345 = 6.0 * t4349 * t13343;
    let t13346 = t1016 * t3418;
    let t13348 = 4.0 * t1382 * t13346;
    let t13349 = t2355 * t3599;
    let t13350 = t3599 * t921;
    let t13352 = 2.0 * t1382 * t13350;
    let t13354 = 0.35750489951850426669e0 * t895 * t11402;
    (t13334, t13336, t13338, t13340, t13342, t13343, t13345, t13346, t13348, t13349, t13350, t13352, t13354)
}
