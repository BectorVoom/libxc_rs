//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 881/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk881<F: Float>(t1106: F, t1181: F, t12936: F, t991: F, t1090: F, t3361: F, t1530: F, t3402: F, t922: F, t944: F, t1172: F, t12935: F) -> (F, F, F, F, F) {
    let t12939 = t12936 * t1181 * t991 * t1106;
    let t12943 = t3361 * t1181 * t991 * t1090;
    let t12945 = t1530 * t3402;
    let t12946 = t944 * t922;
    let t12991 = t12935 * t1172;
    (t12939, t12943, t12945, t12946, t12991)
}
