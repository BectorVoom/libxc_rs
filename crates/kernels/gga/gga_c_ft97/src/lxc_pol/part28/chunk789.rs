//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 789/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk789<F: Float>(t165: F, t7312: F, t379: F, t9073: F, t7339: F, t1969: F, t5935: F, t5968: F, t604: F, t7390: F) -> (F, F, F, F, F, F) {
    let t32717 = t7312 * t165;
    let t32719 = t9073 * t32717 * t379;
    let t32722 = t7339 * t165;
    let t32723 = t32722 * t379;
    let t32724 = t1969 * t32723;
    let t32727 = t5935 * t5968;
    let t32729 = t7390 * t604;
    (t32717, t32719, t32722, t32724, t32727, t32729)
}
