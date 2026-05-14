//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 690/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk690<F: Float>(t352: F, t35589: F, t739: F, t4793: F, t669: F, t131: F, t1338: F, t2019: F, t640: F, t7764: F, t1343: F, t2084: F, t7765: F, t1330: F, t28: F, t271: F, t7553: F, t7557: F) -> (F, F, F, F, F, F, F) {
    let t35590 = t35589 * t352;
    let t35591 = t739 * t35590;
    let t35593 = t4793 * t669;
    let t35604 = t131 * t1338;
    let t35607 = t2019 * t7764 * t640 * t35604;
    let t35608 = 0.45731474687362542471e-3 * t35607;
    let t35611 = t2019 * t2084 * t1343 * t7765;
    let t35612 = 0.24390119833260022651e-2 * t35611;
    let t35613 = t28 * t1330;
    let t35616 = t7553 * t35613 * t271 * t7557;
    (t35590, t35591, t35593, t35604, t35608, t35612, t35616)
}
