//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1261/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1261<F: Float>(t26189: F, t46862: F, t463: F, t6524: F, t1332: F, t38651: F, t6484: F, t8232: F, t26135: F, t8392: F, t26364: F, t1882: F, t26395: F, t1307: F, t8417: F, t26173: F) -> (F, F, F, F, F, F, F, F, F) {
    let t103821 = t46862 * t26189;
    let t103823 = t463 * t6524;
    let t103827 = t38651 * t1332;
    let t103832 = t8232 * t6484;
    let t103835 = 4.0 / 27.0 * t8392 * t26135;
    let t103837 = 2.0 / 27.0 * t8392 * t26364;
    let t103840 = 4.0 / 9.0 * t1882 * t26395;
    let t103849 = t8417 * t1307;
    let t103855 = 4.0 / 3.0 * t8392 * t26173;
    (t103821, t103823, t103827, t103832, t103835, t103837, t103840, t103849, t103855)
}
