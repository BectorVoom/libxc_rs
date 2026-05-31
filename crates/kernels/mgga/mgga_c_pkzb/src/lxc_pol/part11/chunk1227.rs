//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1227/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1227<F: Float>(t10867: F, t17577: F, t17579: F, t683: F, t7411: F, t9216: F, t7483: F, t9219: F, t20911: F, t9222: F, t10892: F, t5776: F) -> (F, F, F, F, F) {
    let t30259 = F::cast_from(0.24955700379505800916e5_f64) * t17577 * t10867 * t17579 * t683;
    let t30261 = F::cast_from(18.0_f64) * t7411 * t9216;
    let t30263 = F::cast_from(12.0_f64) * t7483 * t9219;
    let t30265 = F::cast_from(0.2894756309764656312e3_f64) * t20911 * t9222;
    let t30268 = F::cast_from(24.0_f64) * t5776 * t10892 * t683;
    (t30259, t30261, t30263, t30265, t30268)
}
