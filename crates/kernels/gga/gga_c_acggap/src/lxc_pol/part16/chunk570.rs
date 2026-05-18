//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 570/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk570<F: Float>(t1162: F, t2450: F, t1111: F, t1165: F, t4417: F, t3391: F, t1008: F, t1441: F, t1456: F, t1462: F, t1005: F, t1434: F) -> (F, F, F, F, F, F, F) {
    let t4735 = t2450 * t1162;
    let t4741 = t1165 * t4417 * t1111;
    let t4742 = t3391 * t4741;
    let t4745 = F::new(0.34299214494455789578e-2) * t1008 * t1441;
    let t4747 = F::new(0.17149607247227894789e-2) * t1008 * t1456;
    let t4748 = t1008 * t1462;
    let t4750 = t1005 * t1434;
    (t4735, t4741, t4742, t4745, t4747, t4748, t4750)
}
