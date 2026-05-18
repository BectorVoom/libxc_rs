//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1229/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1229<F: Float>(t10497: F, t2183: F, t11068: F, t27002: F, t7788: F, t11178: F, t1250: F, t251: F, t11061: F, t7790: F, t27013: F, t3489: F) -> (F, F, F, F, F) {
    let t92581 = t2183 * t10497;
    let t92587 = t7788 * t11068 * t27002;
    let t92590 = t11178 * t251 * t1250;
    let t92600 = t7788 * t11061 * t7790;
    let t92604 = t27013 * t3489;
    (t92581, t92587, t92590, t92600, t92604)
}
