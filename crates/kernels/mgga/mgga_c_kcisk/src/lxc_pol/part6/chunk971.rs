//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 971/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk971<F: Float>(t2670: F, t8472: F, t564: F, t22167: F, t2356: F, t8473: F, t2059: F, t7706: F) -> (F, F, F, F) {
    let t30146 = t8472 * t2670;
    let t30147 = t564 * t30146;
    let t30148 = F::new(3.0) / F::new(16.0) * t30147;
    let t30149 = F::new(3.0) * t22167;
    let t30150 = t2356 * t8473;
    let t30151 = F::new(3.0) / F::new(16.0) * t30150;
    let t30153 = t7706 * t2059;
    (t30148, t30149, t30151, t30153)
}
