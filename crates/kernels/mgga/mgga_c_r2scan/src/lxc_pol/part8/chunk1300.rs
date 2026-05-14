//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1300/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1300<F: Float>(t6165: F, t6398: F, t9380: F, t146: F, t2145: F, t3177: F, t2151: F, t3090: F, t560: F, t22790: F, t6086: F, t2155: F, t30856: F, t6063: F, t28005: F, t8077: F) -> (F, F, F, F, F, F) {
    let t31057 = t6165 * t6398 * t9380;
    let t31060 = t146 * t2145 * t3177;
    let t31061 = t31060 * t2151;
    let t31064 = t3090 * t560;
    let t31066 = t22790 * t6086 * t31064;
    let t31069 = t2155 * t6063 * t30856;
    let t31072 = t2155 * t8077 * t28005;
    (t31057, t31060, t31061, t31066, t31069, t31072)
}
