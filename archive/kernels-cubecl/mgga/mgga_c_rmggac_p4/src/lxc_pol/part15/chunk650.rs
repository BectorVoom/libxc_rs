//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 650/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk650<F: Float>(t1982: F, t2314: F, t7428: F, t2191: F, t2283: F, t495: F, t570: F, t515: F, t1971: F, t7230: F, t498: F, t7231: F) -> (F, F, F, F, F) {
    let t9040 = t2314 * t7428 * t1982;
    let t9042 = t2191 * t2283;
    let t9044 = t570 * t495;
    let t9045 = t515 * t9044;
    let t9046 = t1971 * t9045;
    let t9047 = t7230 * t9046;
    let t9049 = t570 * t498;
    let t9050 = t515 * t9049;
    let t9051 = t7231 * t9050;
    (t9040, t9042, t9046, t9047, t9051)
}
