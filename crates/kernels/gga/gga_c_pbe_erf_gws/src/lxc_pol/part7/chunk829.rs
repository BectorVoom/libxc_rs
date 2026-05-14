//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 829/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk829<F: Float>(t16999: F, t17042: F, t184: F, t203: F, t221: F, t4913: F, t4935: F, t4879: F, t1627: F, t4930: F, t4883: F, t16945: F, t16948: F, t16953: F, t16955: F, t16957: F, t16959: F) -> (F, F, F, F, F, F) {
    let t17047 = 2.0 / 15.0 * t203 * (t16999 + t17042) * t184 * t221;
    let t17048 = t4913 * t4935;
    let t17049 = 128.0 / 45.0 * t17048;
    let t17051 = 64.0 / 15.0 * t4913 * t4879;
    let t17053 = 32.0 / 15.0 * t1627 * t4930;
    let t17055 = 32.0 / 15.0 * t1627 * t4883;
    let t17056 = -t16945 - t16948 - t16953 + t16955 - t16957 - t16959 + t17047 + t17049 - t17051 + t17053 + t17055;
    (t17047, t17049, t17051, t17053, t17055, t17056)
}
