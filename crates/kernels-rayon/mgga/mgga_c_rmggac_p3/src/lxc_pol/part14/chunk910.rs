//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 910/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk910(t1986: f64, t5142: f64, t675: f64, t2289: f64, t7944: f64, t1971: f64, t27326: f64, t3351: f64, t7262: f64, t511: f64, t618: f64, t7231: f64, t848: f64) -> (f64, f64, f64, f64) {
    let t39715 = t675 * t1986 * t5142;
    let t39717 = t7944 * t2289;
    let t39721 = t3351 * t1971 * t7262 * t27326;
    let t39726 = t3351 * t7231 * t511 * t618 * t848;
    (t39715, t39717, t39721, t39726)
}
