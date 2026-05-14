//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 518/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk518<F: Float>(t3670: F, t425: F, t431: F, t438: F, t3237: F, t1005: F, t993: F, t174: F, t3101: F, t386: F, t387: F, t384: F, t3669: F, t381: F, t390: F, t1015: F, t144: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3671 = t3670 * t425;
    let t3673 = t3670 * t431;
    let t3677 = t3670 * t438;
    let t3679 = t3237 * t425;
    let t3694 = 0.64311027177104605458e-3 * t1005 * t993;
    let t3695 = t174 * t3101;
    let t3697 = t386 * t387 * t3695;
    let t3699 = 0.21437009059034868486e-3 * t384 * t3697;
    let t3700 = t381 * t3669;
    let t3702 = 0.17006693853500995666e-1 * t3700 * t390;
    let t3706 = 1.0 / t1015 / t144;
    (t3671, t3673, t3677, t3679, t3694, t3697, t3699, t3702, t3706)
}
