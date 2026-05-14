//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 702/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk702<F: Float>(t1225: F, t2258: F, t1012: F, t1224: F, t3367: F, t2251: F, t1121: F, t404: F, t3362: F, t1251: F, t3172: F, t1247: F, t1032: F, t1204: F, t1246: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3688 = t1225 * t2258;
    let t3689 = t1012 * t3688;
    let t3692 = t1224 * t3367;
    let t3693 = t3692 * t2251;
    let t3694 = t1012 * t3693;
    let t3698 = 1.0 / t404 / t1121;
    let t3699 = t3698 * t3362;
    let t3700 = t3699 * t2251;
    let t3701 = t1012 * t3700;
    let t3704 = t3172 * t1251;
    let t3705 = t1247 * t3704;
    let t3707 = t1204 * t1032;
    let t3708 = t3707 * t1246;
    (t3688, t3689, t3693, t3694, t3698, t3700, t3701, t3704, t3705, t3707, t3708)
}
