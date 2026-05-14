//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 837/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk837<F: Float>(t16784: F, t7504: F, t1343: F, t13998: F, t3657: F, t4815: F, t10493: F, t4819: F, t10188: F, t10348: F, t13649: F, t13651: F, t13653: F, t13699: F, t13701: F, t16716: F, t16730: F, t16732: F, t16734: F, t16737: F, t7656: F, t7657: F) -> (F, F, F, F, F) {
    let t16820 = t16784 * t7504;
    let t16824 = 3.0 * t13998 * t1343;
    let t16826 = 3.0 * t3657 * t4815;
    let t16828 = 0.48245472966453314466e2 * t10493 * t4819;
    let t16841 = 0.5477111111111111111e-1 * t13649 - 0.32862666666666666666e0 * t13651 + 0.16431333333333333333e0 * t13653 - 0.76790625e-1 * t16716 - 0.27385555555555555556e0 * t10348 + 0.3071625e0 * t16730 - 0.28483875e1 * t16732 + 0.46074375e0 * t16734 - 0.39862222222222222223e0 * t10188 - t7656 - t7657 + 0.142419375e1 * t16737 + 0.19931111111111111111e0 * t13699 - 0.59793333333333333333e0 * t13701;
    (t16820, t16824, t16826, t16828, t16841)
}
