//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 327/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk327(t225: f64, t1171: f64, t3128: f64, t3119: f64) -> (f64, f64, f64, f64) {
    let t3129 = f64::sqrt(t225);
    let t3131 = 1.0_f64 / t3129 / t1171;
    let t3132 = t3128 * t3131;
    let t3133 = t3132 * t3119;
    (t3129, t3131, t3132, t3133)
}
