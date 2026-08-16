//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1204/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1204(t17381: f64, t20662: f64, t20685: f64, t20834: f64, t20837: f64, t20849: f64, t20892: f64, t20895: f64, t20898: f64, t20900: f64, t20902: f64, t20904: f64, t20905: f64, t20908: f64, t20913: f64, t20916: f64, t5883: f64, t5887: f64, t5894: f64, t721: f64) -> f64 {
    let t20917 = -t20662 - 0.31168546390226634765e3_f64 * t20834 * t5894 + 0.30762056574649219974e4_f64 * t20837 * t17381 * t721 + t20685 - 0.19751673498613801407e-1_f64 * t20849 - t20892 + t20895 - t20898 - t20900 - t20902 - t20904 + 18.0_f64 * t20905 * t5883 - 0.57895126195293126243e3_f64 * t20908 * t5887 + t20913 - t20916;
    t20917
}
