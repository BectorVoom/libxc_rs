//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 541/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk541<F: Float>(t3391: F, t4741: F, t1008: F, t1441: F, t1456: F, t1462: F, t1005: F, t1434: F, t1137: F, t1503: F, t3114: F, t355: F, t352: F, t1427: F, t721: F, t1049: F, t1483: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4742 = t3391 * t4741;
    let t4745 = 0.34299214494455789578e-2 * t1008 * t1441;
    let t4747 = 0.17149607247227894789e-2 * t1008 * t1456;
    let t4748 = t1008 * t1462;
    let t4750 = t1005 * t1434;
    let t4785 = 7.0 / 72.0 * t1137 * t1503;
    let t4794 = t3114 * t355;
    let t4795 = t352 * t4794;
    let t4796 = t1427 * t721;
    let t4797 = t4795 * t4796;
    let t4798 = 0.2445e0 * t4797;
    let t4799 = t1049 * t1483;
    (t4742, t4745, t4747, t4748, t4750, t4785, t4797, t4798, t4799)
}
