//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 976/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk976<F: Float>(t2320: F, t36520: F, t2310: F, t7921: F, t2289: F, t35277: F, t9005: F, t9128: F, t4895: F, t645: F, t1550: F, t11905: F, t2061: F) -> (F, F, F, F, F, F, F) {
    let t40679 = t36520 * t2320;
    let t40681 = t7921 * t2310;
    let t40683 = t35277 * t2289;
    let t40685 = t9128 * t9005;
    let t40687 = t645 * t4895;
    let t40688 = t1550 * t40687;
    let t40690 = t11905 * t2061;
    (t40679, t40681, t40683, t40685, t40687, t40688, t40690)
}
