//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 818/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk818<F: Float>(t3906: F, t9664: F, t1357: F, t4132: F, t689: F, t4131: F, t676: F, t123: F, t3915: F, t2453: F, t3914: F, t1444: F, t2438: F) -> (F, F, F, F, F, F) {
    let t9666 = F::cast_from(0.46263278077393568556e-2_f64) * t3906 * t9664;
    let t9667 = t1357 * t4132;
    let t9668 = t689 * t9667;
    let t9670 = t676 * t4131;
    let t9671 = t123 * t9670;
    let t9672 = t3915 * t9671;
    let t9674 = t2453 * t3914;
    let t9675 = t2438 * t1444;
    (t9666, t9668, t9671, t9672, t9674, t9675)
}
