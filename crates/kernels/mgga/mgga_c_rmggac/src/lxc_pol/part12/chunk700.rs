//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 700/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk700<F: Float>(t35781: F, t2046: F, t2051: F, t271: F, t4773: F, t71: F, t2157: F, t4968: F, t638: F, t7292: F, t7385: F, t2067: F, t25640: F, t2078: F, t3851: F, t7834: F, t797: F) -> (F, F, F, F, F, F, F) {
    let t35782 = 0.44715219694310041527e-2 * t35781;
    let t35786 = t2046 * t4773 * t271 * t71 * t2051;
    let t35787 = 0.16432021104515675446e-2 * t35786;
    let t35795 = t4968 * t2157;
    let t35798 = t638 * t7292 * t7385;
    let t35799 = 0.12195059916630011326e-2 * t35798;
    let t35810 = t25640 * t2067;
    let t35815 = t3851 * t2078;
    let t35824 = t797 * t7834;
    (t35782, t35787, t35795, t35799, t35810, t35815, t35824)
}
