//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 851/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk851<F: Float>(t204: F, t47803: F, t587: F, t2487: F, t6711: F, t4130: F, t46849: F, t4781: F, t590: F, t13750: F, t1441: F, t1339: F, t13749: F, t1537: F, t2478: F, t3695: F, t6576: F) -> (F, F, F, F, F, F) {
    let t47805 = t587 * t204 * t47803;
    let t47808 = t2487 * t6711 * t47803;
    let t47812 = t4781 * t4130 * t46849 * t590;
    let t47823 = 0.51123901271894332902e0 * t1441 * t13750 * t590;
    let t47827 = 0.51123901271894332902e0 * t1537 * t1339 * t13749 * t590;
    let t47829 = t6576 * t3695 * t2478;
    (t47805, t47808, t47812, t47823, t47827, t47829)
}
