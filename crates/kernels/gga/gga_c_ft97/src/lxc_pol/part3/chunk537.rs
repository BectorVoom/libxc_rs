//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 537/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk537<F: Float>(t4805: F, t605: F, t144: F, t1053: F, t3578: F, t1017: F, t1060: F, t574: F, t167: F, t4714: F, t920: F, t2222: F, t2221: F, t2211: F, t2210: F, t4458: F, t569: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4806 = t605 * t4805;
    let t4807 = t144 * t4806;
    let t4810 = t3578 * t1053;
    let t4811 = t144 * t4810;
    let t4815 = t574 * t1060 * t1017;
    let t4819 = t574 * t167 * t4714;
    let t4822 = t920 * t1017;
    let t4823 = t2222 * t4822;
    let t4824 = t2221 * t4823;
    let t4827 = t920 * t1053;
    let t4828 = t2211 * t4827;
    let t4829 = t2210 * t4828;
    let t4833 = t569 * t167 * t4458;
    (t4806, t4807, t4810, t4811, t4815, t4819, t4822, t4823, t4824, t4828, t4829, t4833)
}
