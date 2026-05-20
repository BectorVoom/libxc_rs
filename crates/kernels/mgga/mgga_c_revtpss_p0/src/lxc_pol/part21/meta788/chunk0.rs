//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2836/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2836<F: Float>(t15177: F, t698: F, t15180: F, t15129: F, t2258: F, t141: F, t2908: F, t11144: F, t2251: F, t4186: F, t11341: F, t51851: F) -> (F, F, F, F, F, F, F) {
    let t51921 = t698 * t15177;
    let t51923 = t698 * t15180;
    let t51925 = t15129 * t2258;
    let t51927 = t141 * t2908 * t51925;
    let t51930 = t11144 * t4186 * t2251;
    let t51932 = t141 * t11341 * t51930;
    let t51935 = t141 * t2908 * t51851;
    (t51921, t51923, t51925, t51927, t51930, t51932, t51935)
}
