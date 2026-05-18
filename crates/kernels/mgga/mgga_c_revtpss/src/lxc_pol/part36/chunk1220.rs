//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1220/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1220<F: Float>(t2453: F, t25398: F, t10982: F, t1949: F, t9646: F, t10985: F, t25422: F, t25335: F, t9303: F, t1959: F, t41117: F, t68: F, t785: F) -> (F, F, F, F, F, F) {
    let t93194 = t2453 * t25398;
    let t93206 = F::new(0.19637199382202157274e-3) * t9646 * t1949 * t10982;
    let t93210 = F::new(0.46263278077393568556e-2) * t25422 * t10985;
    let t93224 = F::new(0.26019841438354088051e-2) * t9303 * t25335;
    let t93231 = F::new(0.81814717454467823679e-4) * t41117 * t1959;
    let t93238 = t68 * t785;
    (t93194, t93206, t93210, t93224, t93231, t93238)
}
