//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1095/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1095<F: Float>(t4343: F, t4542: F, t2404: F, t5966: F, t14613: F, t162: F, t4403: F, t14312: F, t5940: F, t705: F, t707: F, t10605: F, t6002: F, t2411: F, t6079: F, t10446: F, t5819: F) -> (F, F, F, F, F, F, F, F) {
    let t18253 = t4542 * t4343;
    let t18256 = t2404 * t5966;
    let t18259 = t14613 * t162;
    let t18261 = 24.0 * t18259 * t4403;
    let t18262 = 2.0 * t14312;
    let t18263 = t705 * t5940;
    let t18265 = 4.0 * t18263 * t707;
    let t18267 = 12.0 * t10605 * t6002;
    let t18268 = t6079 * t2411;
    let t18272 = t10446 * t5819;
    (t18253, t18256, t18261, t18262, t18265, t18267, t18268, t18272)
}
