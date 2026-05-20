//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 868/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk868<F: Float>(t1235: F, t3678: F, t221: F, t462: F, t696: F, t461: F, t1226: F, t140: F, t1222: F, t1225: F, t2258: F, t1012: F) -> (F, F, F, F, F, F, F) {
    let t3679 = t1235 * t3678;
    let t3682 = t221 * t696 * t462;
    let t3684 = t461 * t3682 / F::new(432.0);
    let t3685 = t140 * t1226;
    let t3686 = t1222 * t3685;
    let t3688 = t1225 * t2258;
    let t3689 = t1012 * t3688;
    (t3679, t3682, t3684, t3685, t3686, t3688, t3689)
}
