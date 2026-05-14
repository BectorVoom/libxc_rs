//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 741/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk741<F: Float>(t12663: F, t12707: F, t12774: F, t12996: F, t13037: F, t13081: F, t13184: F, t13225: F, t12939: F, t160: F, t1022: F, t8787: F, t1952: F, t3413: F, t12633: F, t12637: F, t12665: F, t13030: F, t13136: F, t13180: F, t149: F, t165: F, t3313: F, t614: F) -> (F,) {
    let t13228 = t12663 + t12707 + t12774 + t12996 + t13037 + t13081 + t13184 + t13225;
    let t13230 = t12939 * t160;
    let t13234 = t8787 * t1022;
    let t13239 = t1952 * t3413;
    let t13245 = -t13228 * t149 - t13234 * t165 - 2.0 * t13239 * t165 - 2.0 * t3313 * t614 - 4.0 * t12633 - 2.0 * t12637 + 4.0 * t12665 - 2.0 * t13030 - 2.0 * t13136 + 8.0 * t13180 + 2.0 * t13230;
    (t13245,)
}
