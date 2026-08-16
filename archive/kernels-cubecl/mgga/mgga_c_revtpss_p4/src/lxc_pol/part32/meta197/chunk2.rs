//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 876/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk876<F: Float>(t1250: F, t482: F, t5284: F, t1042: F, t1038: F, t1802: F, t1244: F, t1241: F) -> (F, F, F, F) {
    let t5286 = t482 * t5284 * t1250;
    let t5287 = t1042 * t5286;
    let t5291 = t1802 * t1038;
    let t5292 = t1244 * t5291;
    let t5293 = t1241 * t5292;
    (t5286, t5287, t5292, t5293)
}
