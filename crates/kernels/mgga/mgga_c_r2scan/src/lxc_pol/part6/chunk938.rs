//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 938/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk938<F: Float>(t1783: F, t879: F, t2288: F, t6007: F, t2054: F, t761: F, t6044: F, t758: F, t2049: F, t864: F, t2287: F, t244: F, t2279: F, t2292: F, t357: F, t366: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6801 = t879 * t1783;
    let t6804 = t2288 * t6007;
    let t6806 = t2054 * t761;
    let t6809 = t758 * t6044;
    let t6813 = t864 * t2049;
    let t6817 = 1.0 / t2287 / t244;
    let t6818 = t6817 * t6007;
    let t6821 = t2279 * t2292;
    let t6826 = t2288 * t2049;
    let t6827 = t761 * t357;
    let t6828 = t6827 * t366;
    (t6801, t6804, t6806, t6809, t6813, t6817, t6818, t6821, t6826, t6827, t6828)
}
