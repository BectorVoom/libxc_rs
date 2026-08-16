//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 793/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk793(t21713: f64, t68651: f64, t9217: f64, t68422: f64, t9105: f64, t9110: f64, t236: f64, t446: f64, t551: f64, t21714: f64, t68421: f64, t15220: f64, t7720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74302 = t21713 * t68651 * t9217;
    let t74305 = t21713 * t68422 * t9105;
    let t74309 = t21713 * t68422 * t9110;
    let t74312 = t236 * t551 * t446;
    let t74314 = t68421 * t21714 * t74312;
    let t74316 = t7720 * t15220;
    (t74302, t74305, t74309, t74312, t74314, t74316)
}
