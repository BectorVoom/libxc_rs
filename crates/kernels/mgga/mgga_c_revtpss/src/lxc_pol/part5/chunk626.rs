//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 626/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk626<F: Float>(t1444: F, t676: F, t123: F, t3915: F, t1363: F, t2470: F, t1362: F, t1386: F, t820: F, t843: F) -> (F, F, F, F, F, F) {
    let t3916 = t676 * t1444;
    let t3917 = t123 * t3916;
    let t3918 = t3915 * t3917;
    let t3920 = t1363 * t2470;
    let t3922 = 0.13009920719177044025e-1 * t1362 * t3920;
    let t3930 = t820 * t1386 * t843;
    (t3916, t3917, t3918, t3920, t3922, t3930)
}
