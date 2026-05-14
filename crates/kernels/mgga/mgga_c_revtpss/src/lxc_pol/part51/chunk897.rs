//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 897/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk897<F: Float>(t33872: F, t33902: F, t6985: F, t7742: F, t7935: F, t8568: F, t196: F, t197: F, t7894: F, t2035: F, t7898: F, t8600: F, t8596: F, t1883: F, t32195: F, t5673: F) -> (F, F, F, F, F, F, F, F) {
    let t33903 = t33872 + t33902;
    let t33906 = t6985 * t7742;
    let t33910 = t8568 * t7935;
    let t33913 = t7894 * t196 * t197;
    let t33914 = t33913 * t2035;
    let t33916 = t7898 * t8600;
    let t33920 = t7898 * t8596;
    let t33922 = t5673 * t32195 * t1883;
    (t33903, t33906, t33910, t33913, t33914, t33916, t33920, t33922)
}
