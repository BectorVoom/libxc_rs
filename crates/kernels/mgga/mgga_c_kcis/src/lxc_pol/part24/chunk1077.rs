//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1077/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1077<F: Float>(t95903: F, t11081: F, t26960: F, t28106: F, t7772: F, t96727: F, t7794: F, t993: F, t2888: F, t27028: F, t15573: F, t28178: F, t7788: F, t28183: F, t11061: F, t8090: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t97031 = 0.15476481481481481481e-2 * t95903;
    let t97051 = 0.7722800925925925926e-4 * t26960 * t11081 * t28106;
    let t97060 = 0.92754700520833333333e-4 * t7772 * t96727;
    let t97083 = t993 * t7794;
    let t97089 = t2888 * t7794;
    let t97093 = t993 * t27028;
    let t97102 = 0.46336805555555555556e-3 * t7788 * t15573 * t28178;
    let t97103 = t15573 * t28183;
    let t97105 = 0.23168402777777777778e-3 * t7788 * t97103;
    let t97106 = t7772 * t97103;
    let t97153 = t7788 * t11061 * t8090;
    (t97031, t97051, t97060, t97083, t97089, t97093, t97102, t97105, t97106, t97153)
}
