//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 882/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk882<F: Float>(t13715: F, t30391: F, t4129: F, t20292: F, t20373: F, t26138: F, t26150: F, t26159: F, t26176: F, t26179: F, t30288: F, t30292: F, t30296: F, t30300: F, t30303: F, t30327: F, t30340: F) -> (F, F) {
    let t30403 = t13715 * t30391;
    let t30404 = t30403 * t4129;
    let t30421 = -0.60384999999999999999e0 * t30296 + 0.181155e1 * t30303 - 0.5519e0 * t20373 - 0.40256666666666666668e0 * t20292 - 0.412621875e-1 * t30327 + 0.258925e1 * t30340 + 0.11038e0 * t26176 - 0.66228e0 * t26179 - 0.60385000000000000001e0 * t26150 + 0.30192500000000000001e0 * t26159 + 0.20128333333333333333e0 * t26138 - 0.33547222222222222222e0 * t30288 + 0.12077e1 * t30292 - 0.181155e1 * t30300;
    (t30404, t30421)
}
