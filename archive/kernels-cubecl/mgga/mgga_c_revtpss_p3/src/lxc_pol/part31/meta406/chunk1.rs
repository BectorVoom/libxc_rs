//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1455/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1455<F: Float>(t4343: F, t4542: F, t2404: F, t5966: F, t14613: F, t162: F, t4403: F, t14312: F, t5940: F, t705: F, t707: F, t10605: F, t6002: F) -> (F, F, F, F, F, F) {
    let t18253 = t4542 * t4343;
    let t18256 = t2404 * t5966;
    let t18259 = t14613 * t162;
    let t18261 = F::cast_from(24.0_f64) * t18259 * t4403;
    let t18262 = F::cast_from(2.0_f64) * t14312;
    let t18263 = t705 * t5940;
    let t18265 = F::cast_from(4.0_f64) * t18263 * t707;
    let t18267 = F::cast_from(12.0_f64) * t10605 * t6002;
    (t18253, t18256, t18261, t18262, t18265, t18267)
}
