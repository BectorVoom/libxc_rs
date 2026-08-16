//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 825/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk825(t38471: f64, t7473: f64, t2320: f64, t36520: f64, t2310: f64, t7921: f64, t118: f64, t1986: f64, t495: f64, t571: f64, t2001: f64, t498: f64) -> (f64, f64, f64, f64, f64) {
    let t40661 = t38471 * t7473;
    let t40679 = t36520 * t2320;
    let t40681 = t7921 * t2310;
    let t40694 = t1986 * t118 * t571 * t495;
    let t40699 = t2001 * t118 * t571 * t498;
    (t40661, t40679, t40681, t40694, t40699)
}
