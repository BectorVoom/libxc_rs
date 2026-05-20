//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1163/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1163<F: Float>(t25081: F, t8763: F, t136: F, t8736: F, t10309: F, t2247: F, t26754: F, t32801: F, t10301: F, t45963: F, t32805: F, t116: F, t33285: F) -> (F, F, F, F, F, F, F, F, F) {
    let t122820 = t8763 * t25081;
    let t122885 = t8736 * t136;
    let t122886 = t10309 * t122885;
    let t122890 = t2247 * t26754 * t136;
    let t122893 = t10309 * t32801;
    let t122901 = t10301 * t32801;
    let t122911 = t45963 * t8736;
    let t122918 = t10309 * t32805;
    let t124169 = t33285 * t116;
    (t122820, t122885, t122886, t122890, t122893, t122901, t122911, t122918, t124169)
}
