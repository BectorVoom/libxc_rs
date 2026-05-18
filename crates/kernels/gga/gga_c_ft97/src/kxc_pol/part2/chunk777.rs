//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 777/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk777<F: Float>(t3637: F, t8675: F, t358: F, t3653: F, t363: F, t2266: F, t1580: F, t3635: F, t1073: F, t1557: F, t1559: F, t8654: F) -> (F, F, F, F) {
    let t12174 = F::new(4.0) / F::new(9.0) * t8675 * t3637;
    let t12175 = t3653 * t358;
    let t12176 = t12175 * t363;
    let t12177 = t2266 * t12176;
    let t12181 = t2266 * t3635 * t1580;
    let t12184 = t1073 * t1557;
    let t12186 = t8654 * t12184 * t1559;
    (t12174, t12177, t12181, t12186)
}
