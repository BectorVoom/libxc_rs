//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 749/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk749<F: Float>(t5147: F, t970: F, t10450: F, t1856: F, t10464: F, t706: F, t5152: F, t960: F, t11545: F, t11548: F, t11550: F, t11553: F, t11556: F, t11559: F, t11562: F, t11564: F, t158: F, t165: F, t173: F) -> F {
    let t11566 = t970 * t5147;
    let t11568 = t1856 * t10450;
    let t11571 = t706 * t10464;
    let t11574 = t960 * t5152;
    let t11576 = F::cast_from(0.4755e-2_f64) * t165 * t11545 + F::cast_from(0.70578375e-4_f64) * t11548 + F::cast_from(0.30247875e-4_f64) * t173 * t11550 - F::cast_from(0.2016525e-4_f64) * t173 * t11553 + F::cast_from(0.3513e-2_f64) * t158 * t11556 + F::cast_from(0.21078e-1_f64) * t158 * t11559 + F::cast_from(0.117630625e-3_f64) * t11562 - F::cast_from(0.352891875e-4_f64) * t11564 + F::cast_from(0.4705225e-4_f64) * t11566 + F::cast_from(0.50413125e-5_f64) * t173 * t11568 + F::cast_from(0.22405833333333333333e-5_f64) * t173 * t11571 + F::cast_from(0.14052e-1_f64) * t11574;
    t11576
}
