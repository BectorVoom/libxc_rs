//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2932/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2932<F: Float>(t14066: F, t213: F, t14109: F, t47603: F, t9681: F, t14268: F, t3915: F, t686: F, t72: F, t14293: F, t9664: F, t1444: F, t2782: F, t4075: F, t556: F, t5774: F) -> (F, F, F, F, F) {
    let t47909 = t213 * t14066;
    let t47913 = t47603 * t14109 * t9681;
    let t47918 = t3915 * t14268 * t72 * t686;
    let t47920 = t14293 * t9664;
    let t47926 = t2782 * t556 * t4075 * t5774 * t1444;
    (t47909, t47913, t47918, t47920, t47926)
}
