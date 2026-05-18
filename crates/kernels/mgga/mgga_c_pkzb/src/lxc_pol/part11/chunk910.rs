//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 910/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk910<F: Float>(t3069: F, t3073: F, t2240: F, t3739: F, t6201: F, t851: F, t6199: F, t3807: F, t889: F, t1209: F, t3135: F, t3823: F) -> (F, F, F, F, F, F, F, F) {
    let t9864 = t3073 * t3069;
    let t9866 = F::new(0.32163958997385070134e2) * t2240 * t9864;
    let t9867 = t3739 * t6201;
    let t9868 = t9867 * t851;
    let t9870 = F::new(0.51726012919273400301e3) * t6199 * t9868;
    let t9875 = t3807 * t889;
    let t9878 = t1209 * t3135;
    let t9881 = t3823 * t889;
    (t9864, t9866, t9867, t9868, t9870, t9875, t9878, t9881)
}
