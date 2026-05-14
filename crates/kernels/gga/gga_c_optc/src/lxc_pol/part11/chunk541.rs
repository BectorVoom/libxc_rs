//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 541/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk541<F: Float>(t1540: F, t2586: F, t1170: F, t1528: F, t2367: F, t1150: F, t1129: F, t1545: F, t1157: F, t1533: F, t1567: F, t176: F, t1219: F) -> (F, F, F, F, F, F, F, F) {
    let t4509 = t2586 * t1540;
    let t4510 = t1170 * t4509;
    let t4512 = t2367 * t1528;
    let t4513 = t1150 * t4512;
    let t4515 = t1545 * t1129;
    let t4517 = t1533 * t1157;
    let t4535 = t176 * t1567;
    let t4536 = t4535 * t1219;
    (t4509, t4510, t4512, t4513, t4515, t4517, t4535, t4536)
}
