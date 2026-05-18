//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1087/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1087<F: Float>(t118: F, t2001: F, t38523: F, t570: F, t7720: F, t40001: F, t9222: F, t10199: F, t275: F, t42085: F, t8443: F, t1986: F, t571: F, t615: F) -> (F, F, F, F, F) {
    let t47813 = t2001 * t118 * t38523 * t570;
    let t47814 = t7720 * t47813;
    let t47816 = t9222 * t40001;
    let t47818 = t275 * t10199;
    let t47821 = t42085 * t8443;
    let t47825 = t1986 * t118 * t571 * t615;
    (t47814, t47816, t47818, t47821, t47825)
}
