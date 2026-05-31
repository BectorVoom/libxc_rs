//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 502/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk502<F: Float>(t3670: F, t480: F, t221: F, t462: F, t696: F, t461: F, t1224: F, t3367: F, t1121: F, t404: F, t3362: F, t1234: F, t1260: F) -> (F, F, F, F, F, F, F) {
    let t3671 = t3670 * t480;
    let t3682 = t221 * t696 * t462;
    let t3684 = t461 * t3682 / F::cast_from(432.0_f64);
    let t3692 = t1224 * t3367;
    let t3698 = F::cast_from(1.0_f64) / t404 / t1121;
    let t3699 = t3698 * t3362;
    let t3711 = t1234 * t1260;
    (t3671, t3682, t3684, t3692, t3698, t3699, t3711)
}
