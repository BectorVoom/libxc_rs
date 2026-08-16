//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 785/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk785<F: Float>(t7419: F, t969: F, t825: F, t2685: F, t2684: F, t5638: F, t60: F) -> (F, F, F) {
    let t7420 = t969 * t7419;
    let t7421 = t825 * t7420;
    let t7423 = t2685 * t7419;
    let t7424 = t2684 * t7423;
    let t7426 = t5638 * t60;
    (t7421, t7424, t7426)
}
