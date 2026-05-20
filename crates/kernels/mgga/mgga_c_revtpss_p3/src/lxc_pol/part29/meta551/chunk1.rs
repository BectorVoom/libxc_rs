//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1889/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1889<F: Float>(t10073: F, t25937: F, t7282: F, t7506: F, t26069: F, t96255: F, t2453: F, t3908: F, t7507: F, t2435: F, t26301: F, t7289: F, t96276: F) -> (F, F, F, F, F) {
    let t96398 = t10073 * t7282 * t25937 * t7506;
    let t96401 = F::cast_from(0.91399340044406952588e-2_f64) * t26069 * t96255;
    let t96403 = t2453 * t7507 * t3908;
    let t96410 = t2435 * t26301;
    let t96412 = t7289 * t96276;
    (t96398, t96401, t96403, t96410, t96412)
}
