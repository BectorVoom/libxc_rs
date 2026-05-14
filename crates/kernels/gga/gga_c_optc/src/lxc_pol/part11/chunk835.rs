//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 835/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk835<F: Float>(t16784: F, t2476: F, t10188: F, t10348: F, t13649: F, t13651: F, t13653: F, t13699: F, t13701: F, t16716: F, t16730: F, t16732: F, t16734: F, t16737: F, t7593: F, t7594: F) -> (F, F) {
    let t16785 = t16784 * t2476;
    let t16800 = 0.5519e-1 * t13649 - 0.33114e0 * t13651 + 0.16557e0 * t13653 - 0.412621875e-1 * t16716 - 0.27595e0 * t10348 + 0.16504875e0 * t16730 - 0.3883875e1 * t16732 + 0.247573125e0 * t16734 - 0.40256666666666666668e0 * t10188 - t7593 - t7594 + 0.19419375e1 * t16737 + 0.20128333333333333333e0 * t13699 - 0.60385000000000000001e0 * t13701;
    (t16785, t16800)
}
