//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1371/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1371<F: Float>(t2250: F, t751: F, t707: F, t2447: F, t706: F) -> (F, F, F, F) {
    let t9909 = t751 * t2250;
    let t9910 = t707 * t9909;
    let t9911 = F::cast_from(12.0_f64) * t9910;
    let t9912 = t706 * t2447;
    (t9909, t9910, t9911, t9912)
}
