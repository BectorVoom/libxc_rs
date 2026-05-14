//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1092/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1092<F: Float>(t1006: F, t3639: F, t4893: F, t11257: F, t4644: F, t1265: F, t1459: F, t3649: F, t3652: F, t11182: F, t11185: F, t11249: F, t25176: F, t11215: F, t13676: F, t13679: F, t520: F) -> (F, F, F, F, F, F) {
    let t35631 = t1006 * t3639 * t4893;
    let t35634 = t11257 * t3639 * t4644;
    let t35638 = t3649 * t1265 * t1459 * t3652;
    let t35640 = t11182 * t11185;
    let t35643 = t25176 * t1459 * t11249;
    let t35647 = t11215 * t13676 * t520 * t13679;
    (t35631, t35634, t35638, t35640, t35643, t35647)
}
