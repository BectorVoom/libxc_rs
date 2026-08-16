//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 850/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk850<F: Float>(t1396: F, t6927: F, t4123: F, t1464: F, t2001: F, t5632: F, t1468: F, t3754: F, t6281: F) -> (F, F, F, F, F, F, F) {
    let t6928 = t1396 * t6927;
    let t6929 = t4123 * t6928;
    let t6930 = t1464 * t6929;
    let t6932 = t5632 * t2001;
    let t6933 = t1468 * t6932;
    let t6934 = t1464 * t6933;
    let t6937 = t3754 * t6281;
    (t6928, t6929, t6930, t6932, t6933, t6934, t6937)
}
