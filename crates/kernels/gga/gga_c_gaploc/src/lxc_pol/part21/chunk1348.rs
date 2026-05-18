//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1348/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1348<F: Float>(t35043: F, t1339: F, t31585: F, t1537: F, t590: F, t31590: F, t10474: F, t4428: F, t30830: F, t7967: F, t913: F, t10609: F, t31054: F) -> (F, F, F, F, F, F) {
    let t35044 = F::new(0.59584149919750711116e-1) * t35043;
    let t35045 = t1339 * t31585;
    let t35048 = F::new(0.51123901271894332902e1) * t1537 * t35045 * t590;
    let t35052 = F::new(0.51123901271894332902e1) * t1537 * t1339 * t31590 * t590;
    let t35054 = F::new(0.2044956050875773316e1) * t4428 * t10474;
    let t35074 = t30830 * t913 * t7967;
    let t35075 = F::new(0.59584149919750711116e-1) * t35074;
    let t35089 = t31054 * t10609;
    (t35044, t35048, t35052, t35054, t35075, t35089)
}
