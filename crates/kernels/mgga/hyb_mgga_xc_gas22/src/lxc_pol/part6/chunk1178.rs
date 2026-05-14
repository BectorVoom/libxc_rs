//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1178/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1178<F: Float>(t547: F, t9824: F, t10082: F, t19: F, t550: F, t9850: F, t1181: F, t8157: F, t8160: F, t7884: F, t9858: F, t1230: F, t125: F, t19557: F, t19568: F, t19571: F, t19574: F, t19577: F, t19579: F, t19664: F, t22991: F, t22994: F, t22997: F, t27007: F, t2986: F, t555: F, t557: F) -> (F,) {
    let t27015 = t547 * t9824;
    let t27018 = t19 * t550 * t10082;
    let t27021 = t19 * t550 * t9850;
    let t27023 = t1181 * t8157;
    let t27025 = t1181 * t8160;
    let t27027 = t7884 * t9858;
    let t27034 = -5.0 / 432.0 * t22991 + t22994 / 72.0 + t22997 / 72.0 + t27007 / 288.0 + t19557 + t19568 / 48.0 - 5.0 / 144.0 * t19571 - 5.0 / 144.0 * t19574 + t19577 / 96.0 - 5.0 / 144.0 * t19579 + t19664 / 48.0 - t27015 / 32.0 - t27018 / 32.0 - t27021 / 32.0 - t27023 / 16.0 - t27025 / 16.0 + 7.0 / 16.0 * t27027 - t555 * t2986 * t557 * t1230 * t125 / 16.0;
    (t27034,)
}
