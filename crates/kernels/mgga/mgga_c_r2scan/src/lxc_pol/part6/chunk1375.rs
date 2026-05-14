//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1375/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1375<F: Float>(t2207: F, t7088: F, t785: F, t788: F, t20481: F, t551: F, t566: F, t910: F, t2169: F, t7234: F, t1620: F, t8263: F, t2201: F, t2837: F, t5177: F, t22709: F, t5108: F, t7333: F) -> (F, F, F, F, F, F) {
    let t26097 = t2207 * t785 * t788 * t7088;
    let t26106 = t566 * t551 * t20481 * t910;
    let t26108 = t2169 * t7234;
    let t26109 = 0.38140175656238781678e1 * t26108;
    let t26115 = t1620 * t8263;
    let t26116 = 0.17563392970889009434e0 * t26115;
    let t26118 = t2201 * t2837 * t5177;
    let t26119 = 0.2037639021386884617e0 * t26118;
    let t26124 = t5108 * t22709 * t7333;
    (t26097, t26106, t26109, t26116, t26119, t26124)
}
