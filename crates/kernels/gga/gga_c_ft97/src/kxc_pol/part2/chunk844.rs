//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 844/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk844<F: Float>(t12663: F, t12707: F, t12774: F, t12996: F, t13037: F, t13081: F, t13184: F, t13225: F, t12939: F, t160: F, t1022: F, t8787: F) -> (F, F, F) {
    let t13228 = t12663 + t12707 + t12774 + t12996 + t13037 + t13081 + t13184 + t13225;
    let t13230 = t12939 * t160;
    let t13234 = t8787 * t1022;
    (t13228, t13230, t13234)
}
