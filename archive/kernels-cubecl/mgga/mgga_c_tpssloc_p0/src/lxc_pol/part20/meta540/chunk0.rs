//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2081/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2081<F: Float>(t9879: F, t9888: F, t2430: F, t9912: F, t2528: F, t9716: F, t10047: F, t225: F, t9587: F, t9585: F, t10108: F, t257: F) -> (F, F, F, F, F, F, F) {
    let t40806 = t9879 * t9888;
    let t40808 = t9912 * t2430;
    let t40817 = t9716 * t2528;
    let t40852 = t10047 * t225;
    let t40870 = t9587 * t225;
    let t40875 = t9585 * t225;
    let t40889 = F::cast_from(1.0_f64) / t10108 / t257;
    (t40806, t40808, t40817, t40852, t40870, t40875, t40889)
}
