//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 690/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk690<F: Float>(t1068: F, t8640: F, t171: F, t7741: F, t11: F, t41: F, t3630: F, t3637: F, t8675: F, t358: F, t3653: F, t363: F, t2266: F, t1580: F, t3635: F, t1073: F, t1557: F) -> (F, F, F, F, F, F, F) {
    let t12165 = t8640 * t1068;
    let t12168 = 1.0 / t171 / t7741;
    let t12169 = t11 * t12168;
    let t12170 = t41 * t12169;
    let t12171 = t12170 * t3630;
    let t12174 = 4.0 / 9.0 * t8675 * t3637;
    let t12175 = t3653 * t358;
    let t12176 = t12175 * t363;
    let t12177 = t2266 * t12176;
    let t12181 = t2266 * t3635 * t1580;
    let t12184 = t1073 * t1557;
    (t12165, t12170, t12171, t12174, t12177, t12181, t12184)
}
