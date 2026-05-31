//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 930/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk930<F: Float>(t835: F, t9266: F, t1977: F, t3223: F, t11862: F, t160: F, t1983: F, t27: F, t34: F, t1435: F, t5075: F, t1438: F, t1593: F) -> (F, F, F, F, F, F) {
    let t12460 = t9266 * t835;
    let t12461 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t12460;
    let t12462 = t3223 * t1977;
    let t12463 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t12462;
    let t12465 = t160 * t11862 * t1983;
    let t12514 = t27 * t34;
    let t12516 = t5075 * t12514 * t1435;
    let t12519 = t1593 * t1438;
    (t12461, t12463, t12465, t12514, t12516, t12519)
}
