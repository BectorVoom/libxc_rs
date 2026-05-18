//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1032/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1032<F: Float>(t101: F, t8449: F, t203: F, t8958: F, t103: F, t567: F, t1303: F, t147: F, t19: F, t3156: F, t4864: F, t1908: F, t3940: F) -> (F, F, F, F, F) {
    let t25042 = t8449 * t101;
    let t25045 = t8958 * t203;
    let t25047 = t25045 * t103 * t567;
    let t25054 = t3156 * t1303 * t19 * t147;
    let t25076 = t4864 * t19;
    let t25110 = t3940 * t1908;
    (t25042, t25047, t25054, t25076, t25110)
}
