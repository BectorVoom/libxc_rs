//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 855/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk855<F: Float>(t1007: F, t7106: F, t1968: F, t3080: F, t7105: F, t800: F, t3244: F, t7111: F, t3111: F, t7132: F, t1058: F, t7126: F) -> (F, F, F, F, F, F) {
    let t25535 = t7106 * t1007;
    let t25538 = t1968 * t3080 / F::cast_from(432.0_f64);
    let t25539 = t7105 * t800;
    let t25543 = t7111 * t3244;
    let t25551 = t7132 * t3111;
    let t25557 = t7126 * t1058;
    (t25535, t25538, t25539, t25543, t25551, t25557)
}
