//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 734/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk734<F: Float>(t116: F, t3241: F, t3242: F, t11899: F, t2849: F, t115: F, t1497: F, t2770: F, t3209: F, t1724: F, t1540: F, t7878: F, t1170: F, t1528: F, t7274: F, t1150: F) -> (F, F, F, F, F, F, F, F) {
    let t12567 = t3241 * t3242 * t116;
    let t12568 = t11899 * t2849;
    let t12577 = t1497 * t2770 * t115;
    let t12578 = t3209 * t12577;
    let t12581 = t1724 * t12577;
    let t12594 = t7878 * t1540;
    let t12595 = t1170 * t12594;
    let t12597 = t7274 * t1528;
    let t12598 = t1150 * t12597;
    (t12567, t12568, t12578, t12581, t12594, t12595, t12597, t12598)
}
