//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1848/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1848<F: Float>(t45972: F, t7342: F, t10309: F, t26178: F, t94973: F, t530: F, t7535: F, t198: F, t206: F, t7427: F, t25373: F, t26550: F) -> (F, F, F, F, F, F) {
    let t95316 = t45972 * t7342;
    let t95319 = t10309 * t26178;
    let t95397 = F::new(308.0) / F::new(27.0) * t94973;
    let t95472 = t530 * t7535;
    let t95511 = t198 * t206 * t7427;
    let t95536 = t25373 * t26550;
    (t95316, t95319, t95397, t95472, t95511, t95536)
}
