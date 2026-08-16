//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 866/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk866<F: Float>(t2888: F, t9297: F, t2739: F, t2889: F, t3646: F, t5974: F, t2104: F, t3679: F, t5965: F, t2105: F, t2030: F, t2916: F) -> (F, F, F, F, F, F, F, F) {
    let t9298 = t2888 * t9297;
    let t9301 = t2889 * t2739;
    let t9302 = t2888 * t9301;
    let t9307 = t5974 * t3646;
    let t9308 = t2104 * t9307;
    let t9310 = t3679 * t5965;
    let t9311 = t2105 * t9310;
    let t9314 = t2030 * t2916;
    (t9298, t9301, t9302, t9307, t9308, t9310, t9311, t9314)
}
