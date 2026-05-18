//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 879/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk879<F: Float>(t2051: F, t2066: F, t2055: F, t6002: F, t2054: F, t2061: F, t1546: F, t4293: F, t6917: F, t4292: F, t2039: F, t6016: F) -> (F, F, F, F, F, F, F) {
    let t7292 = t2051 * t2066;
    let t7294 = t6002 * t2055;
    let t7296 = t2061 * t2054;
    let t7297 = t1546 * t7296;
    let t7299 = t4293 * t6917;
    let t7300 = t4292 * t7299;
    let t7302 = t6016 * t2039;
    (t7292, t7294, t7296, t7297, t7299, t7300, t7302)
}
