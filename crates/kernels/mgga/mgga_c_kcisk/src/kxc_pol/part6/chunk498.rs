//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 498/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk498<F: Float>(t3845: F, t698: F, t445: F, t5082: F, t213: F, t695: F, t1849: F, t967: F, t167: F, t4597: F, t1797: F, t704: F) -> (F, F, F, F, F, F) {
    let t5126 = t3845 * t698;
    let t5128 = F::new(0.16804375e-4) * t445 * t5126;
    let t5129 = F::new(0.23911438650126355246e-1) * t5082;
    let t5134 = t213 * t695;
    let t5135 = F::new(0.15538616723388920628e-3) * t5134;
    let t5136 = t967 * t1849;
    let t5168 = t167 * t4597;
    let t5180 = t1797 * t704;
    (t5128, t5129, t5135, t5136, t5168, t5180)
}
