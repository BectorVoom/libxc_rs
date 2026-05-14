//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 889/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk889<F: Float>(t9725: F, t9728: F, t999: F, t292: F, t737: F, t285: F, t1071: F, t240: F, t9: F, t109: F, t992: F, t995: F, t991: F, t2909: F, t993: F, t1000: F, t2888: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9851 = 0.93932222222222222223e0 * t9725;
    let t9852 = 0.36793333333333333333e0 * t9728;
    let t9873 = t999 * t999;
    let t9874 = 1.0 / t9873;
    let t9881 = t737 * t292;
    let t9883 = 5.0 / 1296.0 * t285 * t9881;
    let t9896 = 1.0 / t240 / t1071;
    let t9897 = t9 * t9896;
    let t9916 = t109 * t992;
    let t9917 = t9916 * t995;
    let t9918 = t991 * t9917;
    let t9924 = t993 * t2909;
    let t9933 = t2888 * t1000;
    (t9851, t9852, t9874, t9883, t9897, t9916, t9918, t9924, t9933)
}
