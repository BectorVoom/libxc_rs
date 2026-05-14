//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 933/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk933<F: Float>(t12589: F, t4376: F, t4380: F, t4396: F, t4567: F, t3382: F, t4402: F, t4894: F, t997: F, t1576: F, t3228: F, t1581: F, t1008: F, t4849: F, t4853: F, t4878: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17586 = t12589 * t4376;
    let t17592 = t4396 * t4380;
    let t17605 = t4396 * t4567;
    let t17607 = t3382 * t4402;
    let t17613 = t997 * t4894;
    let t17615 = t3228 * t1576;
    let t17617 = t3228 * t1581;
    let t17619 = t1008 * t4849;
    let t17621 = t1008 * t4853;
    let t17623 = t1008 * t4878;
    (t17586, t17592, t17605, t17607, t17613, t17615, t17617, t17619, t17621, t17623)
}
