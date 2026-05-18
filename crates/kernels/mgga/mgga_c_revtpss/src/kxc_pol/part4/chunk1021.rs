//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1021/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1021<F: Float>(t2645: F, t2723: F, t10115: F, t253: F, t233: F, t2760: F, t869: F, t689: F, t2777: F, t2789: F, t2439: F, t2435: F, t2790: F) -> (F, F, F, F, F) {
    let t10943 = t2723 * t2645;
    let t10948 = F::new(0.11044544084478153697e-3) * t10115 * t253;
    let t10959 = t233 * t2760;
    let t10960 = t869 * t10959;
    let t10961 = t689 * t10960;
    let t10963 = t2777 * t2789;
    let t10964 = t2439 * t10963;
    let t10966 = t2435 * t2790;
    (t10943, t10948, t10961, t10964, t10966)
}
