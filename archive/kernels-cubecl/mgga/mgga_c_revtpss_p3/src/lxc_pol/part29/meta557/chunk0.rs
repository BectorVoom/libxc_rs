//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1899/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1899<F: Float>(t1444: F, t5659: F, t1955: F, t25949: F, t1883: F, t4131: F, t1903: F, t3923: F, t4003: F, t2453: F, t27883: F, t27836: F, t4075: F) -> (F, F, F, F, F, F, F) {
    let t97839 = t5659 * t1444;
    let t97855 = t1955 * t25949;
    let t97858 = t1883 * t4131;
    let t97870 = t1903 * t3923;
    let t97871 = t97870 * t4003;
    let t97916 = t2453 * t27883;
    let t97933 = t1955 * t27836 * t4075;
    (t97839, t97855, t97858, t97870, t97871, t97916, t97933)
}
