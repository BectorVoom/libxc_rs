//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 937/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk937<F: Float>(t25081: F, t8763: F, t33553: F, t575: F, t1464: F, t8970: F, t136: F, t33362: F, t10309: F, t2247: F, t10301: F, t33358: F, t45972: F, t45963: F, t116: F, t33374: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t122820 = t8763 * t25081;
    let t124440 = t33553 * t575;
    let t124442 = t8970 * t1464;
    let t124455 = t33362 * t136;
    let t124456 = t10309 * t124455;
    let t124463 = t2247 * t124455;
    let t124480 = t10301 * t33362;
    let t124483 = t45972 * t33358;
    let t124503 = t45963 * t33358;
    let t124508 = t10301 * t33358;
    let t124533 = t33374 * t116;
    (t122820, t124440, t124442, t124456, t124463, t124480, t124483, t124503, t124508, t124533)
}
