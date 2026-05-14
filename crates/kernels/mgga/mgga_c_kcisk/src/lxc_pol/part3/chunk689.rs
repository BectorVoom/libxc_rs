//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 689/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk689<F: Float>(t11545: F, t11548: F, t11550: F, t11553: F, t11556: F, t11559: F, t11562: F, t11564: F, t11566: F, t11568: F, t11571: F, t11574: F, t158: F, t165: F, t173: F, t5155: F, t960: F) -> (F, F) {
    let t11576 = 0.4755e-2 * t165 * t11545 + 0.70578375e-4 * t11548 + 0.30247875e-4 * t173 * t11550 - 0.2016525e-4 * t173 * t11553 + 0.3513e-2 * t158 * t11556 + 0.21078e-1 * t158 * t11559 + 0.117630625e-3 * t11562 - 0.352891875e-4 * t11564 + 0.4705225e-4 * t11566 + 0.50413125e-5 * t173 * t11568 + 0.22405833333333333333e-5 * t173 * t11571 + 0.14052e-1 * t11574;
    let t11578 = t960 * t5155;
    (t11576, t11578)
}
