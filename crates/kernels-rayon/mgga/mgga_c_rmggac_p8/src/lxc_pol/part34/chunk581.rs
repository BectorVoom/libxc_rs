//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 581/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk581(t14116: f64, t15067: f64, t2868: f64, t3072: f64, t3065: f64, t558: f64) -> (f64, f64, f64) {
    let t15068 = t14116 * t15067;
    let t15070 = t2868 * t3072;
    let t15071 = 0.2993560425465952141e-1_f64 * t15070;
    let t15075 = t3065 * t558;
    (t15068, t15071, t15075)
}
