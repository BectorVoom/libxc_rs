//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1022/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1022<F: Float>(t32094: F, t3266: F, t38921: F, t5674: F, t25996: F, t5675: F, t8411: F, t25878: F, t3052: F, t7824: F, t22952: F, t25883: F) -> (F, F, F, F) {
    let t144866 = t5674 * t38921 * t32094 * t3266;
    let t144870 = t5674 * t8411 * t5675 * t25996;
    let t144874 = t25878 * t7824 * t32094 * t3052;
    let t144878 = t22952 * t8411 * t32094 * t25883;
    (t144866, t144870, t144874, t144878)
}
