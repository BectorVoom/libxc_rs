//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1219/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1219<F: Float>(t26182: F, t26234: F, t26473: F, t26858: F, t1135: F, t508: F, t1027: F, t11985: F, t3116: F, t12068: F, t4386: F, t8498: F, t3117: F, t8914: F, t438: F, t935: F) -> (F, F, F, F, F) {
    let t26860 = t26182 + t26234 + t26473 + t26858;
    let t26869 = t508 * t1135;
    let t26870 = t26869 * t1027;
    let t26872 = t3116 * t26870 * t11985;
    let t26878 = t4386 * t12068 * t8498;
    let t26880 = t3117 * t8914;
    let t26881 = t935 * t438;
    (t26860, t26872, t26878, t26880, t26881)
}
