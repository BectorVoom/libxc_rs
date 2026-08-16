//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 980/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk980<F: Float>(t1042: F, t2943: F, t3093: F, t932: F, t9725: F, t2861: F, t3184: F, t3217: F, t982: F, t1130: F, t2865: F, t1014: F, t3241: F) -> (F, F, F, F, F, F, F) {
    let t10202 = t2943 * t1042;
    let t10208 = t932 * t3093;
    let t10218 = F::cast_from(0.12841111111111111111e-1_f64) * t9725;
    let t10243 = t2861 * t3184;
    let t10245 = t982 * t3217;
    let t10250 = t2865 * t1130;
    let t10255 = t1014 * t3241;
    (t10202, t10208, t10218, t10243, t10245, t10250, t10255)
}
