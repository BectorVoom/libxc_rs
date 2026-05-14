//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1220/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1220<F: Float>(t146: F, t6533: F, t978: F, t20481: F, t551: F, t566: F, t910: F, t2169: F, t7234: F, t1620: F, t8263: F, t2201: F, t2837: F, t5177: F, t2195: F, t7983: F) -> (F, F, F, F, F, F) {
    let t26088 = t146 * t6533 * t978;
    let t26106 = t566 * t551 * t20481 * t910;
    let t26108 = t2169 * t7234;
    let t26109 = 0.38140175656238781678e1 * t26108;
    let t26115 = t1620 * t8263;
    let t26116 = 0.17563392970889009434e0 * t26115;
    let t26118 = t2201 * t2837 * t5177;
    let t26119 = 0.2037639021386884617e0 * t26118;
    let t26141 = t2195 * t7983;
    (t26088, t26106, t26109, t26116, t26119, t26141)
}
