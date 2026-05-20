//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1114/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1114<F: Float>(t1312: F, t2055: F, t2322: F, t5523: F, t670: F, t7357: F, t7359: F, t7373: F, t2106: F, t531: F) -> (F, F) {
    let t7484 = F::new(2.0) * t1312 * t7373 + F::new(2.0) * t2055 * t2322 + F::new(2.0) * t2055 * t5523 + F::new(2.0) * t670 * t7359 + t7357;
    let t7488 = t531 * t2106;
    (t7484, t7488)
}
