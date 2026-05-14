//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 828/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk828<F: Float>(t1592: F, t4786: F, t3092: F, t1058: F, t1660: F, t1053: F, t1659: F, t225: F, t4743: F, t366: F, t1065: F, t2857: F, t4181: F, t1042: F, t2852: F, t3181: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4787 = t1592 * t4786;
    let t4788 = t3092 * t4787;
    let t4792 = t1660 * t1058;
    let t4794 = t1659 * t1053;
    let t4797 = t4743 * t225;
    let t4798 = t4797 * t366;
    let t4801 = t1065 * t2857;
    let t4802 = t4801 * t4181;
    let t4803 = t1042 * t4802;
    let t4806 = t3181 * t2852;
    (t4787, t4788, t4792, t4794, t4797, t4798, t4801, t4802, t4803, t4806)
}
