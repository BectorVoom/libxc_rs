//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 742/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk742<F: Float>(t2120: F, t4690: F, t4694: F, t4681: F, t7037: F, t2164: F, t4712: F, t4715: F, t4685: F, t7122: F, t4631: F, t7110: F, t108: F, t1256: F, t110: F, t3313: F) -> (F, F, F, F, F, F, F, F) {
    let t13376 = t2120 * t4690;
    let t13378 = t2120 * t4694;
    let t13380 = t7037 * t4681;
    let t13390 = t2164 * t4712;
    let t13392 = t2164 * t4715;
    let t13482 = t7122 * t4685;
    let t13487 = t7110 * t4631;
    let t13502 = t1256 * t108;
    let t13503 = t13502 * t110;
    let t13504 = t3313 * t13503;
    (t13376, t13378, t13380, t13390, t13392, t13482, t13487, t13504)
}
