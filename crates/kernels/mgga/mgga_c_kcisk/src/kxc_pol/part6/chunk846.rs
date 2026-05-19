//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 846/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk846<F: Float>(t28357: F, t4744: F, t10715: F, t6817: F, t8522: F, t2063: F, t7715: F) -> (F, F, F) {
    let t28358 = t28357 * t4744;
    let t28360 = F::cast_from(0.96490945932906628932e2_f64) * t10715 * t28358;
    let t28362 = t6817 * t8522;
    let t28368 = t7715 * t2063;
    (t28360, t28362, t28368)
}
