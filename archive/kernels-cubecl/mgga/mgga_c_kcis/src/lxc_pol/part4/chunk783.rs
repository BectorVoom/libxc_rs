//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 783/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk783<F: Float>(t303: F, t4789: F, t1800: F, t922: F, t3202: F, t3200: F, t1804: F, t3210: F, t1121: F, t1646: F, t3203: F, t1133: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4790 = t303 * t4789;
    let t4792 = t1800 * t922;
    let t4793 = t3202 * t4792;
    let t4794 = t3200 * t4793;
    let t4796 = t1804 * t922;
    let t4797 = t3210 * t4796;
    let t4798 = t3200 * t4797;
    let t4800 = t1646 * t1121;
    let t4801 = t3203 * t4800;
    let t4802 = t3202 * t4801;
    let t4803 = t3200 * t4802;
    let t4805 = t1646 * t1133;
    (t4790, t4792, t4793, t4794, t4796, t4797, t4798, t4801, t4802, t4803, t4805)
}
