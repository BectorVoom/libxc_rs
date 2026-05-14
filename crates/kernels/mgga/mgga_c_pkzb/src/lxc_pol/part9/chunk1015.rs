//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1015/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1015<F: Float>(t17928: F, t5951: F, t197: F, t2023: F, t2079: F, t2096: F, t5921: F, t5925: F, t5928: F, t5708: F, t5713: F, t2011: F, t5939: F, t757: F, t2026: F, t2032: F) -> (F, F, F, F, F, F, F, F) {
    let t18008 = t17928 * t5951;
    let t18009 = t18008 * t197;
    let t18016 = t2079 * t2023;
    let t18024 = t2096 * t5921;
    let t18026 = t5925 * t5928;
    let t18028 = t5713 * t5708;
    let t18033 = t757 * t5939 * t2011;
    let t18036 = t2026 * t5939 * t2032;
    (t18008, t18009, t18016, t18024, t18026, t18028, t18033, t18036)
}
