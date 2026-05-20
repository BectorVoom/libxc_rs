//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1744/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1744<F: Float>(t25431: F, t28368: F, t25411: F, t786: F, t7998: F, t789: F, t231: F, t7997: F, t836: F, t7076: F, t1558: F, t7398: F) -> (F, F, F, F, F, F, F) {
    let t28369 = t25431 * t28368;
    let t28371 = t25411 * t28368;
    let t28373 = t786 * t7998;
    let t28374 = t28373 * t789;
    let t28377 = t7997 * t836 * t231;
    let t28378 = t7076 * t28377;
    let t28384 = t7398 * t1558 * t231;
    (t28369, t28371, t28373, t28374, t28377, t28378, t28384)
}
