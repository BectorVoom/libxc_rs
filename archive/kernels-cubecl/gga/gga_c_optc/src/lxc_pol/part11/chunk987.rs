//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 987/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk987<F: Float>(t18058: F, t9118: F, t9124: F, t5311: F, t8974: F, t4458: F, t9104: F, t18012: F, t4289: F, t18023: F, t3146: F, t894: F) -> (F, F, F, F, F, F, F, F) {
    let t18059 = t18058 * t9118;
    let t18062 = t18058 * t9124;
    let t18065 = t8974 * t5311;
    let t18066 = t4458 * t18065;
    let t18069 = t18058 * t9104;
    let t18072 = t4289 * t18012;
    let t18075 = t3146 * t18023;
    let t18076 = t894 * t18075;
    (t18059, t18062, t18065, t18066, t18069, t18072, t18075, t18076)
}
