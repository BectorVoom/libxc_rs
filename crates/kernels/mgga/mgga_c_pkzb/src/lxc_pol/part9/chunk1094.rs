//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1094/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1094<F: Float>(t17928: F, t288: F, t5950: F, t197: F, t2030: F, t5951: F, t2023: F, t2079: F, t2096: F, t5921: F, t5925: F, t5928: F) -> (F, F, F, F, F, F, F, F) {
    let t17999 = t17928 / t5950 / t288;
    let t18000 = t17999 * t197;
    let t18002 = t2030 * t2030;
    let t18008 = t17928 * t5951;
    let t18009 = t18008 * t197;
    let t18016 = t2079 * t2023;
    let t18024 = t2096 * t5921;
    let t18026 = t5925 * t5928;
    (t17999, t18000, t18002, t18008, t18009, t18016, t18024, t18026)
}
