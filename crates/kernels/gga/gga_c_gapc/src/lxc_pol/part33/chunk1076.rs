//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1076/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1076<F: Float>(t11733: F, t949: F, t1971: F, t9066: F, t2660: F, t8135: F, t11905: F, t18815: F, t11302: F, t15811: F, t18824: F, t7259: F, t8142: F) -> (F, F, F, F, F, F, F) {
    let t33371 = t11733 * t949;
    let t33373 = t1971 * t9066;
    let t33374 = t2660 * t33373;
    let t33375 = t33374 * t8135;
    let t33377 = t11905 * t18815;
    let t33380 = t15811 * t11302 * t18824;
    let t33383 = t7259 * t33373 * t8142;
    (t33371, t33373, t33374, t33375, t33377, t33380, t33383)
}
