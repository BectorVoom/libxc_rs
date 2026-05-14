//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 962/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk962<F: Float>(t1409: F, t167: F, t532: F, t5801: F, t1401: F, t5805: F, t4023: F, t1441: F, t1650: F, t11951: F, t12048: F, t1444: F, t2622: F, t1445: F, t5654: F, t12065: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17057 = t1409 * t167;
    let t17062 = 0.93706135855523581992e-2 * t532 * t5801;
    let t17065 = 0.28111840756657074598e-1 * t1401 * t5805;
    let t17088 = t4023 * t1409;
    let t17096 = t1441 * t1650;
    let t17098 = t11951 * t1650;
    let t17100 = t12048 * t167;
    let t17102 = t2622 * t1444;
    let t17103 = t17102 * t167;
    let t17137 = 0.47822877300252710492e-1 * t1445 * t5654;
    let t17143 = 0.62154466893555682512e-3 * t12065 * t5654;
    (t17057, t17062, t17065, t17088, t17096, t17098, t17100, t17103, t17137, t17143)
}
