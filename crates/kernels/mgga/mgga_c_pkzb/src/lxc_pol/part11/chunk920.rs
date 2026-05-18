//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 920/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk920<F: Float>(t3793: F, t870: F, t2281: F, t3792: F, t3102: F, t3106: F, t3779: F, t6290: F, t6088: F, t6090: F, t7955: F, t8233: F, t9782: F, t9797: F) -> (F, F, F, F, F, F, F) {
    let t10009 = t3793 * t870;
    let t10012 = t3792 * t2281;
    let t10013 = t10012 * t870;
    let t10016 = t3106 * t3102;
    let t10019 = t3779 * t6290;
    let t10020 = t10019 * t870;
    let t10027 = -t6088 + F::new(0.23744444444444444444e-1) * t6090 + F::new(0.47488888888888888888e-1) * t7955 - t8233 - F::new(0.17808333333333333333e-1) * t9782 + F::new(0.53425e-1) * t9797;
    (t10009, t10012, t10013, t10016, t10019, t10020, t10027)
}
