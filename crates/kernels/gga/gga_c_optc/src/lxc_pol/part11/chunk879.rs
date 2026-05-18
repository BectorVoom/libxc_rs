//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 879/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk879<F: Float>(t16729: F, t787: F, t3665: F, t4793: F, t3681: F, t16715: F, t7512: F, t10188: F, t10348: F, t13649: F, t13651: F, t13653: F, t13699: F, t13701: F, t16716: F, t7786: F, t7787: F) -> (F, F, F, F, F) {
    let t16730 = t787 * t16729;
    let t16732 = t3665 * t4793;
    let t16734 = t3681 * t4793;
    let t16737 = t7512 * t16715;
    let t16741 = F::new(0.69463333333333333335e-1) * t13649 - F::new(0.41678000000000000001e0) * t13651 + F::new(0.20839e0) * t13653 - F::new(0.157790625e0) * t16716 - F::new(0.34731666666666666667e0) * t10348 + F::new(0.6311625e0) * t16730 - F::new(0.52945875e1) * t16732 + F::new(0.94674375e0) * t16734 - F::new(0.68863333333333333332e0) * t10188 - t7786 - t7787 + F::new(0.264729375e1) * t16737 + F::new(0.34431666666666666666e0) * t13699 - F::new(0.103295e1) * t13701;
    (t16730, t16732, t16734, t16737, t16741)
}
