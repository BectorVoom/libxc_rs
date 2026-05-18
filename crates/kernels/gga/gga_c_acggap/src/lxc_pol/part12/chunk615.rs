//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 615/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk615<F: Float>(t3391: F, t4741: F, t1008: F, t1441: F, t1456: F, t1462: F, t1005: F, t1434: F, t1524: F, t301: F, t1089: F, t1095: F) -> (F, F, F, F, F, F, F) {
    let t4742 = t3391 * t4741;
    let t4745 = F::new(0.34299214494455789578e-2) * t1008 * t1441;
    let t4747 = F::new(0.17149607247227894789e-2) * t1008 * t1456;
    let t4748 = t1008 * t1462;
    let t4750 = t1005 * t1434;
    let t4752 = t1524 * t301;
    let t4754 = t1089 * t1095 * t4752;
    (t4742, t4745, t4747, t4748, t4750, t4752, t4754)
}
