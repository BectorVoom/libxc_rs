//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 612/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk612<F: Float>(t3223: F, t366: F, t1054: F, t1058: F, t1014: F, t2857: F, t1010: F, t614: F, t1016: F, t140: F, t1011: F, t271: F, t905: F) -> (F, F, F, F, F, F) {
    let t3224 = t3223 * t366;
    let t3234 = t1054 * t1058;
    let t3236 = t1014 * t2857;
    let t3241 = t614 * t1010;
    let t3244 = t140 * t1016;
    let t3245 = t1011 * t3244;
    let t3252 = F::new(1.0) / t271 / t905;
    (t3224, t3234, t3236, t3241, t3245, t3252)
}
