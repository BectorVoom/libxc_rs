//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 735/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk735<F: Float>(t3202: F, t4792: F, t3200: F, t1804: F, t922: F, t3210: F, t1121: F, t1646: F, t3203: F, t1133: F, t3211: F, t1773: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
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
    let t4806 = t3211 * t4805;
    let t4807 = t3210 * t4806;
    let t4808 = t3200 * t4807;
    let t4813 = t1773 * t1133;
    (t4793, t4794, t4796, t4797, t4798, t4801, t4802, t4803, t4806, t4807, t4808, t4813)
}
