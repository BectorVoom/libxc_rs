//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 713/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk713<F: Float>(t2192: F, t3812: F, t2059: F, t3831: F, t2110: F, t3929: F, t2240: F, t4169: F, t19848: F, t492: F, t2250: F, t979: F, t1390: F, t2209: F, t3532: F, t2242: F, t306: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20798 = t3812 * t2192;
    let t20820 = t3831 * t2059;
    let t20886 = t2110 * t3929;
    let t20922 = t2240 * t4169;
    let t21066 = t19848 * t492;
    let t21152 = t979 * t2250;
    let t21230 = t2209 * t1390;
    let t21239 = t2209 * t3532;
    let t21252 = t2242 * t306;
    (t20798, t20820, t20886, t20922, t21066, t21152, t21230, t21239, t21252)
}
