//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1166/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1166<F: Float>(t10437: F, t16089: F, t444: F, t1429: F, t3329: F, t8: F, t3333: F, t983: F, t1430: F, t2499: F, t8657: F, t10444: F, t1435: F) -> (F, F, F, F, F, F) {
    let t28696 = t16089 * t10437 * t444;
    let t28700 = t3329 * t8 * t1429;
    let t28703 = t983 * t3333;
    let t28704 = t28703 * t444;
    let t28707 = t1430 * t3333;
    let t28710 = t2499 * t8657;
    let t28714 = t1435 * t10444 * t444;
    (t28696, t28700, t28704, t28707, t28710, t28714)
}
