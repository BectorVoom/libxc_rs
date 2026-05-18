//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 660/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk660<F: Float>(t3915: F, t3917: F, t1363: F, t2470: F, t1362: F, t1398: F) -> (F, F, F, F) {
    let t3918 = t3915 * t3917;
    let t3920 = t1363 * t2470;
    let t3922 = F::new(0.13009920719177044025e-1) * t1362 * t3920;
    let t3923 = t1398 * t1398;
    (t3918, t3920, t3922, t3923)
}
