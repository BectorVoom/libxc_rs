//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 954/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk954<F: Float>(t11356: F, t3363: F, t9865: F, t33211: F, t7595: F, t28602: F, t3784: F, t3131: F, t8785: F, t1084: F, t15610: F, t1734: F, t8709: F, t15516: F, t3708: F, t9563: F, t9934: F) -> (F, F, F, F, F, F, F, F) {
    let t33405 = t3363 * t11356 * t9865;
    let t33407 = t33211 * t7595;
    let t33409 = t3784 * t28602;
    let t33411 = t3131 * t8785;
    let t33413 = t1084 * t33411 * t15610;
    let t33415 = t1734 * t8709;
    let t33417 = t1084 * t33415 * t15516;
    let t33420 = t9563 * t3708 * t9934;
    (t33405, t33407, t33409, t33411, t33413, t33415, t33417, t33420)
}
