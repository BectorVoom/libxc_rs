//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 435/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk435<F: Float>(t101: F, t814: F, t154: F, t1583: F, t1570: F, t1574: F, t309: F, t3901: F, t4858: F, t4861: F, t4862: F, t4865: F, t4868: F, t4871: F, t4879: F, t4882: F, t4883: F, t4886: F, t538: F, t544: F, t804: F, t822: F, t826: F, t87: F, t98: F) -> (F,) {
    let t4889 = t101 * t814;
    let t4892 = t1583 * t154;
    let t4895 = 400.0 / 27.0 * t804 * t538 - 200.0 / 27.0 * t309 * t1570 - 100.0 / 9.0 * t309 * t1574 - 20.0 / 27.0 * t87 * t4858 + 40.0 / 9.0 * t4861 * t4862 + 20.0 / 9.0 * t87 * t4865 + 10.0 / 3.0 * t87 * t4868 - 10.0 * t87 * t4871 - 100.0 / 27.0 * t544 * t822 - 50.0 / 9.0 * t544 * t826 - 20.0 / 27.0 * t98 * t4879 - 40.0 / 9.0 * t4882 * t4883 + 20.0 / 9.0 * t98 * t4886 - 10.0 / 3.0 * t98 * t4889 + 10.0 * t98 * t4892 + t3901;
    (t4895,)
}
