//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 998/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk998<F: Float>(t1985: F, t22666: F, t33296: F, t214: F, t7918: F, t6907: F, t22704: F, t33249: F, t81326: F, t22633: F, t31550: F, t90566: F) -> (F, F, F, F, F) {
    let t122164 = t1985 * t22666 * t33296;
    let t122166 = t214 * t7918;
    let t122168 = t1985 * t122166 * t6907;
    let t122178 = t22704 * t81326 * t33249;
    let t122187 = t22633 * t90566 * t31550;
    (t122164, t122166, t122168, t122178, t122187)
}
