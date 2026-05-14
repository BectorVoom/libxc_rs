//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 652/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk652<F: Float>(t1248: F, t5464: F, t3781: F, t487: F, t460: F, t3302: F, t471: F, t670: F, t93: F, t198: F, t530: F, t532: F, t539: F, t73: F, t241: F, t4000: F, t820: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5465 = t5464 * t1248;
    let t5477 = t3781 * t487;
    let t5478 = t460 * t5477;
    let t5479 = t3302 * t1248;
    let t5480 = t5479 * t471;
    let t5523 = t93 * t670;
    let t5536 = t198 * t530;
    let t5541 = t198 * t532;
    let t5650 = t539 * t73;
    let t5671 = t820 * t4000 * t241;
    (t5465, t5477, t5478, t5480, t5523, t5536, t5541, t5650, t5671)
}
