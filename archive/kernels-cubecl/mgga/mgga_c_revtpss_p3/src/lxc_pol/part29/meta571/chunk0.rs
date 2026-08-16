//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1918/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1918<F: Float>(t14587: F, t689: F, t27312: F, t1568: F, t7063: F, t25410: F, t25304: F, t27212: F, t27349: F, t25260: F, t4368: F, t820: F, t844: F) -> (F, F, F, F, F, F, F) {
    let t98809 = t14587 * t689;
    let t98815 = t27312 * t689;
    let t98848 = t7063 * t1568;
    let t98849 = t98848 * t25410;
    let t98867 = t25304 * t27212;
    let t98892 = t27349 * t689;
    let t98937 = t820 * t25260 * t844 * t4368;
    (t98809, t98815, t98848, t98849, t98867, t98892, t98937)
}
