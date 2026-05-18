//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1043/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1043<F: Float>(t122: F, t23608: F, t6856: F, t786: F, t4: F, t103: F, t2232: F, t7062: F, t880: F, t4914: F, t572: F, t268: F, t8449: F) -> (F, F, F, F, F, F, F) {
    let t23609 = t23608 * t122;
    let t23612 = t6856 * M_PI * t786;
    let t23624 = t6856 * t4;
    let t23678 = t2232 * t103;
    let t23723 = t880 * t7062;
    let t23726 = t572 * t4914;
    let t24081 = t8449 * t268;
    (t23609, t23612, t23624, t23678, t23723, t23726, t24081)
}
