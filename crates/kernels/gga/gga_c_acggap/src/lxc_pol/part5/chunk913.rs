//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 913/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk913<F: Float>(t1137: F, t4777: F, t4781: F, t4597: F, t4496: F, t1140: F, t4590: F, t4480: F, t3431: F, t4963: F, t3409: F, t4991: F, t12743: F, t1545: F, t13183: F, t1541: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16824 = t1137 * t4777;
    let t16826 = t1137 * t4781;
    let t16839 = t1137 * t4597;
    let t16841 = t1137 * t4496;
    let t16847 = t1140 * t4590;
    let t16849 = t1140 * t4480;
    let t16863 = t3431 * t4963;
    let t16865 = t3409 * t4991;
    let t16867 = t12743 * t1545;
    let t16869 = t13183 * t1541;
    (t16824, t16826, t16839, t16841, t16847, t16849, t16863, t16865, t16867, t16869)
}
