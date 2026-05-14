//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 647/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk647<F: Float>(t1717: F, t331: F, t829: F, t1035: F, t1646: F, t1045: F, t167: F, t313: F, t1027: F, t1728: F, t1727: F, t3073: F, t4670: F, t102: F, t2474: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4831 = t331 * t1717;
    let t4833 = t1717 * t829;
    let t4836 = t1035 * t1646;
    let t4837 = t4836 * t1045;
    let t4840 = t313 * t167;
    let t4843 = t1027 * t1728;
    let t4845 = t1728 * t829;
    let t4848 = t3073 * t1727;
    let t4849 = t4848 * t1045;
    let t4852 = t1035 * t4670;
    let t4858 = t102 * t2474;
    (t4831, t4833, t4836, t4837, t4840, t4843, t4845, t4848, t4849, t4852, t4858)
}
