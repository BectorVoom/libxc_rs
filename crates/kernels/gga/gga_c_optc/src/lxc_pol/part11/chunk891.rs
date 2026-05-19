//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 891/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk891<F: Float>(t16800: F, t16815: F, t837: F, t16784: F, t7504: F, t1343: F, t13998: F, t3657: F, t4815: F, t10493: F, t4819: F, t10188: F, t10348: F, t13649: F, t13651: F, t13653: F, t13699: F, t13701: F, t16716: F, t16730: F, t16732: F, t16734: F, t16737: F, t7656: F, t7657: F) -> (F, F, F, F, F, F, F) {
    let t16816 = t16800 + t16815;
    let t16817 = t16816 * t837;
    let t16820 = t16784 * t7504;
    let t16824 = F::new(3.0) * t13998 * t1343;
    let t16826 = F::new(3.0) * t3657 * t4815;
    let t16828 = F::cast_from(0.48245472966453314466e2_f64) * t10493 * t4819;
    let t16841 = F::cast_from(0.5477111111111111111e-1_f64) * t13649 - F::cast_from(0.32862666666666666666e0_f64) * t13651 + F::cast_from(0.16431333333333333333e0_f64) * t13653 - F::new(0.76790625e-1) * t16716 - F::cast_from(0.27385555555555555556e0_f64) * t10348 + F::new(0.3071625e0) * t16730 - F::new(0.28483875e1) * t16732 + F::new(0.46074375e0) * t16734 - F::cast_from(0.39862222222222222223e0_f64) * t10188 - t7656 - t7657 + F::cast_from(0.142419375e1_f64) * t16737 + F::cast_from(0.19931111111111111111e0_f64) * t13699 - F::cast_from(0.59793333333333333333e0_f64) * t13701;
    (t16816, t16817, t16820, t16824, t16826, t16828, t16841)
}
