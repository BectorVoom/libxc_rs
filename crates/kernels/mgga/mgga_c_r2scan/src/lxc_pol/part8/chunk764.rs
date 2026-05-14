//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 764/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk764<F: Float>(t219: F, t225: F, t5429: F, t234: F, t61: F, t704: F, t1830: F, t695: F) -> (F, F, F, F) {
    let t5431 = t219 * t5429 * t225;
    let t5433 = 0.5848223622634646207e0 * t234 * t5431;
    let t5434 = t61 * t704;
    let t5435 = t1830 * t695;
    (t5431, t5433, t5434, t5435)
}
