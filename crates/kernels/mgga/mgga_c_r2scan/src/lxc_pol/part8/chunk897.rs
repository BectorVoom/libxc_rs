//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 897/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk897<F: Float>(t7624: F, t8077: F, t2155: F, t560: F, t921: F, t6086: F, t6085: F, t7619: F, t6093: F, t1567: F, t2115: F) -> (F, F, F, F, F, F, F, F) {
    let t8078 = t8077 * t7624;
    let t8080 = 0.97574405393827830186e-2 * t2155 * t8078;
    let t8081 = t921 * t560;
    let t8082 = t6086 * t8081;
    let t8084 = 0.11643651550782197811e-1 * t6085 * t8082;
    let t8085 = t6086 * t7619;
    let t8086 = t6093 * t8085;
    let t8088 = t2115 * t1567;
    (t8078, t8080, t8081, t8082, t8084, t8085, t8086, t8088)
}
