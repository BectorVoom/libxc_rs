//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 502/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk502<F: Float>(t136: F, t1413: F, t1353: F, t221: F, t3978: F, t247: F, t2682: F, t550: F, t548: F, t1408: F, t820: F, t843: F) -> (F, F, F, F, F, F) {
    let t3979 = t1413 * t136;
    let t3981 = t3979 * t221 * t1353;
    let t3982 = t3978 * t3981;
    let t3985 = t2682 * t550 * t247;
    let t3987 = F::new(0.56688979511669985553e-2) * t548 * t3985;
    let t3989 = t820 * t1408 * t843;
    (t3979, t3981, t3982, t3985, t3987, t3989)
}
