//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1166/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1166<F: Float>(t17928: F, t5951: F, t197: F, t2023: F, t2079: F, t46: F, t2020: F, t2037: F, t2011: F, t5939: F, t757: F, t2026: F, t2032: F, t2038: F, t2040: F, t1478: F, t301: F) -> (F, F, F, F, F, F, F, F) {
    let t18008 = t17928 * t5951;
    let t18009 = t18008 * t197;
    let t18016 = t2079 * t2023;
    let t18017 = t18016 * t46;
    let t18018 = t2020 * t18017;
    let t18021 = t2037 * t18017;
    let t18033 = t757 * t5939 * t2011;
    let t18036 = t2026 * t5939 * t2032;
    let t18039 = t2038 * t5939 * t2040;
    let t18060 = t1478 * t301;
    (t18008, t18009, t18018, t18021, t18033, t18036, t18039, t18060)
}
