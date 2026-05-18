//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1174/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1174<F: Float>(t13750: F, t1441: F, t590: F, t1339: F, t13749: F, t1537: F, t2478: F, t3695: F, t6576: F, t2482: F, t9263: F, t46850: F, t4820: F, t6824: F) -> (F, F, F, F, F) {
    let t47823 = F::new(0.51123901271894332902e0) * t1441 * t13750 * t590;
    let t47827 = F::new(0.51123901271894332902e0) * t1537 * t1339 * t13749 * t590;
    let t47829 = t6576 * t3695 * t2478;
    let t47832 = t9263 * t3695 * t2482;
    let t47835 = t6824 * t4820 * t46850;
    (t47823, t47827, t47829, t47832, t47835)
}
