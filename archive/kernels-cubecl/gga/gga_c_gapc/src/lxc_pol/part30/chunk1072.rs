//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1072/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1072<F: Float>(t3406: F, t8133: F, t2579: F, t3412: F, t1104: F, t4914: F, t10524: F, t575: F, t2468: F, t3563: F, t1615: F, t3478: F) -> (F, F, F, F, F, F) {
    let t30324 = t3406 * t8133;
    let t30325 = t2579 * t3412 * t30324;
    let t30523 = t1104 * t4914;
    let t30867 = t10524 * t575;
    let t31754 = t3563 * t2468;
    let t31767 = t3478 * t1615;
    (t30324, t30325, t30523, t30867, t31754, t31767)
}
