//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 290/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk290<F: Float>(t918: F, t923: F, t240: F, t696: F, t281: F, t283: F, t346: F) -> (F, F, F, F, F) {
    let t924 = t923 * t918;
    let t926 = t696 * t240;
    let t928 = t281 * t926 * t283;
    let t929 = F::cast_from(0.82156666666666666667e-1_f64) * t928;
    let t930 = t240 * t346;
    (t924, t926, t928, t929, t930)
}
