//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1092/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1092<F: Float>(t34328: F, t7235: F, t651: F, t7002: F, t8065: F, t32387: F, t4248: F, t13426: F, t8641: F, t18227: F, t32401: F, t34258: F, t7374: F, t648: F, t7741: F, t2056: F) -> (F, F, F, F, F, F, F, F, F) {
    let t128284 = t7235 * t34328;
    let t128287 = 2.0 * t651 * t8065 * t7002;
    let t128289 = t4248 * t32387;
    let t128293 = t13426 * t8641;
    let t128294 = t18227 * t8641;
    let t128295 = t4248 * t32401;
    let t128301 = t34258 * t7374;
    let t128302 = t648 * t7741;
    let t128303 = t128302 * t2056;
    (t128284, t128287, t128289, t128293, t128294, t128295, t128301, t128302, t128303)
}
