//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1372/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1372<F: Float>(t12009: F, t3150: F, t1032: F, t3043: F, t1040: F, t1035: F, t11239: F, t342: F, t3145: F, t334: F) -> (F, F, F, F, F) {
    let t12010 = t3150 * t12009;
    let t12020 = t3043 * t1032;
    let t12021 = t12020 * t1040;
    let t12046 = t11239 * t1035;
    let t12047 = t342 * t12046;
    let t12050 = F::new(1.0) / t3145 / t334;
    (t12010, t12021, t12046, t12047, t12050)
}
