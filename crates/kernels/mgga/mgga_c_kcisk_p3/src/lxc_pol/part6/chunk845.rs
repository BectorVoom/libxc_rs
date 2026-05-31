//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 845/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk845<F: Float>(t1685: F, t28341: F, t4787: F, t22760: F, t7509: F, t22891: F, t2382: F, t6802: F, t8574: F, t16356: F, t8577: F, t2381: F, t8549: F) -> (F, F, F, F, F, F) {
    let t28343 = t4787 * t28341 * t1685;
    let t28346 = t22760 * t7509;
    let t28352 = F::cast_from(3.0_f64) * t22891 * t2382;
    let t28354 = F::cast_from(3.0_f64) * t6802 * t8574;
    let t28356 = F::cast_from(0.48245472966453314466e2_f64) * t16356 * t8577;
    let t28357 = t8549 * t2381;
    (t28343, t28346, t28352, t28354, t28356, t28357)
}
