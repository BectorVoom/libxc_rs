//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1261/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1261<F: Float>(t3823: F, t555: F, t6160: F, t547: F, t9824: F, t10082: F, t19: F, t550: F, t9850: F, t1181: F, t8157: F, t8160: F) -> (F, F, F, F, F, F) {
    let t27007 = t555 * t6160 * t3823;
    let t27015 = t547 * t9824;
    let t27018 = t19 * t550 * t10082;
    let t27021 = t19 * t550 * t9850;
    let t27023 = t1181 * t8157;
    let t27025 = t1181 * t8160;
    (t27007, t27015, t27018, t27021, t27023, t27025)
}
