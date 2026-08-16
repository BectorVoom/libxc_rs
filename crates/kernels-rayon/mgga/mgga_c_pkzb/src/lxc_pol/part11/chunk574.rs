//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 574/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk574(t3102: f64, t871: f64, t1196: f64, t2281: f64, t870: f64, t2175: f64, t2285: f64, t3017: f64, t3028: f64) -> (f64, f64, f64, f64) {
    let t3103 = t3102 * t871;
    let t3106 = t1196 * t2281;
    let t3107 = t3106 * t870;
    let t3113 = t2285 - 0.92708333333333333333e-2_f64 * t2175 - 0.92708333333333333333e-2_f64 * t3017 + 0.278125e-1_f64 * t3028;
    (t3103, t3106, t3107, t3113)
}
