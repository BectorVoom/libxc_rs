//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 668/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk668<F: Float>(t567: F, t9295: F, t564: F, t1001: F, t167: F, t2689: F, t1049: F, t116: F, t3182: F, t1008: F, t195: F, t1053: F, t3187: F) -> (F, F, F, F, F, F, F, F) {
    let t9296 = t567 * t9295;
    let t9297 = t564 * t9296;
    let t9345 = t167 * t1001;
    let t9352 = t2689 * t1001;
    let t9355 = t116 * t1049;
    let t10328 = F::cast_from(6.0_f64) * t3182;
    let t10334 = t1008 * t1008;
    let t10335 = F::cast_from(1.0_f64) / t10334;
    let t10336 = t195 * t10335;
    let t10337 = t3187 * t1053;
    (t9296, t9297, t9345, t9352, t9355, t10328, t10336, t10337)
}
