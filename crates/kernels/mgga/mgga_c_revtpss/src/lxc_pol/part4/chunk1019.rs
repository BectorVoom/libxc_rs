//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1019/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1019<F: Float>(t13700: F, t13714: F, t1892: F, t785: F, t1358: F, t2439: F, t1903: F, t4075: F, t1444: F, t556: F, t2782: F, t212: F, t5710: F, t689: F, t4131: F, t4076: F) -> (F, F, F, F, F) {
    let t13716 = t13700 / 2.0 + t13714 / 2.0;
    let t13725 = t785 * t1892;
    let t13726 = t13725 * t1358;
    let t13727 = t2439 * t13726;
    let t13729 = t4075 * t1903;
    let t13730 = t13729 * t1444;
    let t13731 = t556 * t13730;
    let t13733 = 0.21951497276451705328e-1 * t2782 * t13731;
    let t13734 = t212 * t5710;
    let t13735 = t13734 * t1358;
    let t13737 = 0.10975748638225852664e-1 * t689 * t13735;
    let t13738 = t1903 * t4131;
    let t13739 = t4076 * t13738;
    (t13716, t13727, t13733, t13737, t13739)
}
