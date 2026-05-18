//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1235/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1235<F: Float>(t32387: F, t4248: F, t116: F, t34187: F, t13426: F, t8641: F, t18227: F, t32401: F, t34258: F, t7374: F, t648: F, t7741: F) -> (F, F, F, F, F, F, F) {
    let t128289 = t4248 * t32387;
    let t128291 = t34187 * t116;
    let t128293 = t13426 * t8641;
    let t128294 = t18227 * t8641;
    let t128295 = t4248 * t32401;
    let t128301 = t34258 * t7374;
    let t128302 = t648 * t7741;
    (t128289, t128291, t128293, t128294, t128295, t128301, t128302)
}
