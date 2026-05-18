//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 670/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk670<F: Float>(t1899: F, t5183: F, t213: F, t220: F, t1849: F, t579: F, t1336: F, t140: F, t4596: F, t694: F) -> (F, F, F, F, F) {
    let t10426 = t5183 * t1899;
    let t10447 = t220 * t213;
    let t10459 = F::new(1.0) / t579 / t1849;
    let t10461 = t140 * t1336 * t10459;
    let t10463 = F::new(1.0) / t4596 / t694;
    (t10426, t10447, t10459, t10461, t10463)
}
