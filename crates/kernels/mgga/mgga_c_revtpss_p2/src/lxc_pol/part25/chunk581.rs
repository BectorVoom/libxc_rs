//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 581/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk581<F: Float>(t1012: F, t3237: F, t1010: F, t614: F, t1016: F, t140: F, t1011: F, t1015: F, t2258: F, t271: F, t905: F) -> (F, F, F, F, F, F, F) {
    let t3238 = t1012 * t3237;
    let t3241 = t614 * t1010;
    let t3244 = t140 * t1016;
    let t3245 = t1011 * t3244;
    let t3247 = t1015 * t2258;
    let t3248 = t1012 * t3247;
    let t3252 = F::new(1.0) / t271 / t905;
    (t3238, t3241, t3244, t3245, t3247, t3248, t3252)
}
