//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 506/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk506<F: Float>(t110: F, t141: F, t1392: F, t907: F, t106: F, t317: F, t1411: F, t2693: F, t1659: F, t282: F, t115: F, t2770: F, t5: F) -> (F, F, F, F, F, F, F) {
    let t3843 = t141 * t110;
    let t3853 = t1392 * t907;
    let t3860 = t106 * t317;
    let t3861 = t2693 * t1411;
    let t3881 = t1659 * t282;
    let t3882 = t2770 * t115;
    let t3883 = t3882 * t5;
    (t3843, t3853, t3860, t3861, t3881, t3882, t3883)
}
