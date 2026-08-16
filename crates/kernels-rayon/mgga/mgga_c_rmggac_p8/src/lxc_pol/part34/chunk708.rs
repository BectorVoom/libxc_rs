//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 708/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk708(t1330: f64, t1343: f64, t3056: f64, t641: f64, t3046: f64, t507: f64, t7190: f64, t3148: f64, t7716: f64, t16130: f64, t511: f64, t1971: f64) -> (f64, f64, f64, f64) {
    let t69760 = t3056 * t1330 * t1343 * t641;
    let t69788 = t507 * t7190 * t3046;
    let t69806 = t7716 * t3148;
    let t69807 = t511 * t16130;
    let t69808 = t1971 * t69807;
    (t69760, t69788, t69806, t69808)
}
