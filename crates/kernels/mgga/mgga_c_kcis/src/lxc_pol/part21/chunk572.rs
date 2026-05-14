//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 572/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk572<F: Float>(t4621: F, t945: F, t4714: F, t2919: F, t2955: F, t2967: F, t2968: F, t4612: F, t4615: F, t4618: F, t4623: F, t4658: F, t4660: F, t4701: F, t4703: F, t4706: F, t4709: F, t4712: F) -> (F, F, F) {
    let t4715 = t945 * t4621;
    let t4716 = t4714 * t4715;
    let t4718 = -0.9494625e0 * t4658 + 0.1898925e1 * t4660 + t2955 + 0.99655555555555555557e-1 * t2919 + 0.99655555555555555557e-1 * t4612 - 0.19931111111111111111e0 * t4615 + 0.59793333333333333334e0 * t4618 - 0.59793333333333333334e0 * t4623 + 0.15358125e0 * t4701 + 0.3071625e0 * t4703 + t2967 + 0.54771111111111111111e-1 * t2968 + 0.54771111111111111111e-1 * t4706 - 0.27385555555555555556e-1 * t4709 + 0.16431333333333333333e0 * t4712 - 0.16431333333333333333e0 * t4716;
    (t4715, t4716, t4718)
}
