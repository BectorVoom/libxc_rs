//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 374/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk374(t464: f64, t483: f64, t1968: f64, t1966: f64, t1004: f64, t108: f64, t490: f64) -> (f64, f64, f64) {
    let t7242 = t464 * t483;
    let t7243 = t7242 * t1968;
    let t7244 = t1966 * t7243;
    let t7247 = t1004 * t108;
    let t7248 = t490 * t7247;
    (t7244, t7247, t7248)
}
