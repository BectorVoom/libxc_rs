//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1879/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1879<F: Float>(t1468: F, t2411: F, t30: F, t41154: F, t14495: F, t689: F, t14587: F, t27312: F, t1568: F, t7063: F, t25410: F, t25304: F, t27212: F) -> (F, F, F, F, F, F, F, F) {
    let t98658 = t2411 * t1468;
    let t98785 = t41154 * t30;
    let t98801 = t14495 * t689;
    let t98809 = t14587 * t689;
    let t98815 = t27312 * t689;
    let t98848 = t7063 * t1568;
    let t98849 = t98848 * t25410;
    let t98867 = t25304 * t27212;
    (t98658, t98785, t98801, t98809, t98815, t98848, t98849, t98867)
}
