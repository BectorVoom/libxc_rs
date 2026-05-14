//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1451/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1451<F: Float>(t25032: F, t25034: F, t25037: F, t26805: F, t26952: F, t26955: F, t27375: F, t27380: F, t27382: F, t27386: F, t27387: F, t27393: F, t372: F, t881: F, t24409: F, t471: F, t97: F) -> (F, F) {
    let t27394 = -t27375 + t25032 - t25034 - 0.7089e1 * t881 * t26952 - 0.2363e1 * t881 * t26955 - 0.7089e1 * t27380 - 0.14178e2 * t27382 - t27386 + 3.0 * t27387 + t372 * t26805 - t25037 + t27393;
    let t27397 = 3.0 * t97 * t471 * t24409;
    (t27394, t27397)
}
