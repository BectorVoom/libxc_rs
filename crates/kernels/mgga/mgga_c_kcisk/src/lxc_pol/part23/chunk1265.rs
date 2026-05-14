//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1265/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1265<F: Float>(t2736: F, t43108: F, t79: F, t32388: F, t9512: F, t32046: F, t3748: F, t32473: F, t9523: F, t32422: F, t9524: F, t2737: F, t32497: F, t4419: F, t32401: F, t32384: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t109531 = t43108 * t79 * t2736;
    let t109539 = t9512 * t32388;
    let t109541 = t3748 * t32046;
    let t109543 = t32473 * t9523;
    let t109565 = t9512 * t32422;
    let t109567 = t9524 * t32422;
    let t109570 = t2737 * t4419 * t32497;
    let t109575 = t9512 * t32401;
    let t109577 = t9524 * t32401;
    let t109579 = t4419 * t32384;
    (t109531, t109539, t109541, t109543, t109565, t109567, t109570, t109575, t109577, t109579)
}
