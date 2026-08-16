//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 585/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk585(t15098: f64, t3851: f64, t13902: f64, t556: f64, t13905: f64, t2842: f64, t3046: f64, t551: f64) -> (f64, f64, f64, f64) {
    let t15099 = t3851 * t15098;
    let t15101 = t13902 * t556;
    let t15103 = t13905 * t2842;
    let t15105 = t3046 * t551;
    (t15099, t15101, t15103, t15105)
}
