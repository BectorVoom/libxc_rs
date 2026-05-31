//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1356/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1356<F: Float>(t94365: F, t95026: F, t95071: F, t95117: F, t1464: F, t7318: F, t26093: F, t575: F, t10259: F, t572: F, t7330: F, t117: F, t94991: F) -> (F, F, F, F, F) {
    let t95119 = t94365 + t95026 + t95071 + t95117;
    let t95125 = t7318 * t1464;
    let t95127 = t26093 * t575;
    let t95131 = F::cast_from(6.0_f64) * t572 * t7330 * t10259;
    let t95136 = F::cast_from(3.0_f64) * t572 * t117 * t94991;
    (t95119, t95125, t95127, t95131, t95136)
}
