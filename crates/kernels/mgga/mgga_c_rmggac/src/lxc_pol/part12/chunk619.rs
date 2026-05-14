//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 619/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk619<F: Float>(t515: F, t8794: F, t235: F, t2367: F, t874: F, t352: F, t1356: F, t570: F, t7567: F, t1635: F, t880: F, t1971: F, t3351: F, t2144: F, t5898: F, t2289: F, t7720: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8795 = t515 * t8794;
    let t8796 = t235 * t8795;
    let t8800 = t874 * t2367;
    let t8801 = t8800 * t352;
    let t8802 = t1356 * t8801;
    let t8804 = t7567 * t570;
    let t8805 = t1356 * t8804;
    let t8807 = t880 * t1635;
    let t8808 = t1971 * t8807;
    let t8809 = t3351 * t8808;
    let t8811 = t2144 * t5898;
    let t8812 = t1971 * t8811;
    let t8813 = t3351 * t8812;
    let t8815 = t7720 * t2289;
    (t8795, t8796, t8800, t8801, t8802, t8804, t8805, t8808, t8809, t8812, t8813, t8815)
}
