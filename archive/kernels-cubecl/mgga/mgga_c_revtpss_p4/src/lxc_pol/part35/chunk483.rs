//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 483/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk483<F: Float>(t247: F, t2682: F, t550: F, t548: F, t1408: F, t820: F, t843: F, t1386: F, t240: F, t1384: F, t544: F) -> (F, F, F, F, F) {
    let t3985 = t2682 * t550 * t247;
    let t3987 = F::cast_from(0.56688979511669985553e-2_f64) * t548 * t3985;
    let t3989 = t820 * t1408 * t843;
    let t3992 = t1386 * t240;
    let t3999 = F::cast_from(1.0_f64) / t1384 / t544;
    (t3985, t3987, t3989, t3992, t3999)
}
