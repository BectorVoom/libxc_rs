//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 483/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk483<F: Float>(t1046: F, t3172: F, t1041: F, t283: F, t905: F, t1020: F, t1062: F, t1021: F, t1058: F, t371: F, t373: F, t676: F) -> (F, F, F, F, F) {
    let t3173 = t3172 * t1046;
    let t3174 = t1041 * t3173;
    let t3181 = F::cast_from(1.0_f64) / t283 / t905;
    let t3188 = t1020 * t1062;
    let t3194 = t1021 * t1058;
    let t3201 = t371 * t676 * t373;
    (t3174, t3181, t3188, t3194, t3201)
}
