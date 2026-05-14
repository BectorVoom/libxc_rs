//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1184/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1184<F: Float>(t32081: F, t35036: F, t544: F, t2365: F, t31752: F, t4391: F, t549: F, t7025: F, t7906: F, t1339: F, t31585: F, t1537: F, t590: F, t31590: F, t10474: F, t4428: F) -> (F, F, F, F, F, F) {
    let t35037 = t544 * t32081 * t35036;
    let t35038 = 0.10427226235956374445e0 * t35037;
    let t35040 = t4391 * t2365 * t31752;
    let t35041 = 0.17875244975925213335e0 * t35040;
    let t35043 = t7025 * t549 * t7906;
    let t35044 = 0.59584149919750711116e-1 * t35043;
    let t35045 = t1339 * t31585;
    let t35048 = 0.51123901271894332902e1 * t1537 * t35045 * t590;
    let t35052 = 0.51123901271894332902e1 * t1537 * t1339 * t31590 * t590;
    let t35054 = 0.2044956050875773316e1 * t4428 * t10474;
    (t35038, t35041, t35044, t35048, t35052, t35054)
}
