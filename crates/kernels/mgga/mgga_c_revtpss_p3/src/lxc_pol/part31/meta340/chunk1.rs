//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1348/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1348<F: Float>(t1222: F, t13011: F, t3367: F, t404: F, t1204: F, t3140: F, t3599: F, t1242: F, t3603: F, t471: F, t3609: F, t414: F) -> (F, F, F, F, F, F, F) {
    let t13012 = t1222 * t13011;
    let t13026 = F::new(1.0) / t404 / t3367;
    let t13032 = t1204 * t3140;
    let t13033 = t13032 * t3599;
    let t13037 = t1242 * t1242;
    let t13038 = F::new(1.0) / t13037;
    let t13045 = t3603 * t471;
    let t13058 = t13032 * t3609;
    let t13099 = F::new(1.0) / t414 / t3367;
    (t13012, t13026, t13033, t13038, t13045, t13058, t13099)
}
