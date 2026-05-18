//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 712/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk712<F: Float>(t2030: F, t2074: F, t2020: F, t2029: F, t2026: F, t6: F, t616: F, t1948: F, t3440: F, t6318: F, t6321: F, t6324: F, t6328: F, t6330: F, t6342: F, t6356: F, t6526: F, t6613: F, t6619: F, t6621: F, t6623: F) -> (F, F, F, F, F, F, F) {
    let t6797 = t2030 * t2074;
    let t6799 = t2020 * t2029;
    let t6800 = t6799 * t2026;
    let t6802 = t6 * t616;
    let t6803 = t6802 * t1948;
    let t6804 = t3440 * t6803;
    let t6807 = t6318 - t6321 + t6324 - t6328 + t6330 + t6342 + t6526 - t6356 + t6613 - t6619 + t6621 - t6623;
    (t6797, t6799, t6800, t6802, t6803, t6804, t6807)
}
