//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 941/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk941<F: Float>(t2315: F, t874: F, t2316: F, t368: F, t2049: F, t2312: F, t2317: F, t6804: F, t6806: F, t6809: F, t6868: F, t875: F, t158: F, t166: F, t1234: F, t2266: F, t2267: F) -> (F, F, F, F, F) {
    let t6874 = t2315 * t874;
    let t6876 = 1.0 / t2316 / t368;
    let t6879 = 0.1714584e0 * t6804 - 0.1714584e0 * t6806 * t2049 + 0.285764e-1 * t6809 + 0.285764e-1 * t6868 * t875 - 0.857292e-1 * t2312 * t2317 * t874 + 0.571528e-1 * t6874 * t6876;
    let t6880 = t6879 * t158;
    let t6881 = t6880 * t166;
    let t6883 = t2266 * t2267 * t1234;
    (t6876, t6879, t6880, t6881, t6883)
}
