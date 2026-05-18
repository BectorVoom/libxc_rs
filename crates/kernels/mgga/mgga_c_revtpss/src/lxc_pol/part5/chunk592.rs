//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 592/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk592<F: Float>(t3181: F, t66: F, t1020: F, t1062: F, t1021: F, t1058: F, t371: F, t373: F, t676: F, t367: F, t225: F, t3057: F) -> (F, F, F, F, F, F) {
    let t3182 = t66 * t3181;
    let t3188 = t1020 * t1062;
    let t3194 = t1021 * t1058;
    let t3201 = t371 * t676 * t373;
    let t3203 = F::new(0.47637797908966374413e-4) * t367 * t3201;
    let t3204 = t3057 * t225;
    (t3182, t3188, t3194, t3201, t3203, t3204)
}
