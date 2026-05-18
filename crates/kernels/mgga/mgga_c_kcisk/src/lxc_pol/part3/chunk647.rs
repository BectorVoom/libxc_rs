//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 647/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk647<F: Float>(t1757: F, t5063: F, t1899: F, t5062: F, t1869: F, t4581: F, t5048: F, t1799: F, t1894: F, t3293: F, t5185: F, t5184: F) -> (F, F, F, F) {
    let t10381 = t5063 * t1757;
    let t10382 = t1899 * t10381;
    let t10383 = t5062 * t10382;
    let t10384 = t1869 * t10383;
    let t10386 = t4581 * t5048;
    let t10387 = t1799 * t10386;
    let t10389 = t3293 * t1894;
    let t10390 = t5185 * t10389;
    let t10391 = t5184 * t10390;
    (t10381, t10384, t10387, t10391)
}
