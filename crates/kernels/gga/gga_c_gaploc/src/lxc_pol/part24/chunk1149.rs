//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1149/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1149<F: Float>(t20954: F, t3196: F, t1407: F, t9445: F, t1328: F, t20550: F, t6914: F, t9438: F, t1429: F, t549: F, t9572: F, t1323: F, t7033: F, t9439: F) -> (F, F, F, F, F) {
    let t30901 = t20954 * t3196;
    let t30902 = F::new(0.38342925953920749676e0) * t30901;
    let t30903 = t1407 * t9445;
    let t30907 = t6914 * t9438 * t20550 * t1328;
    let t30920 = F::new(0.11916829983950142223e0) * t1429 * t549 * t9572;
    let t30923 = t7033 * t9438 * t9439 * t1323;
    (t30902, t30903, t30907, t30920, t30923)
}
