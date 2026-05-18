//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 555/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk555<F: Float>(t1020: F, t1062: F, t1053: F, t1021: F, t1058: F, t225: F, t3043: F, t366: F, t371: F, t373: F, t676: F, t367: F) -> (F, F, F, F, F, F, F) {
    let t3188 = t1020 * t1062;
    let t3191 = t1020 * t1053;
    let t3194 = t1021 * t1058;
    let t3196 = t3043 * t225;
    let t3197 = t3196 * t366;
    let t3201 = t371 * t676 * t373;
    let t3203 = F::new(0.47637797908966374413e-4) * t367 * t3201;
    (t3188, t3191, t3194, t3196, t3197, t3201, t3203)
}
