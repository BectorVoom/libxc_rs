//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 924/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk924(t225: f64, t3817: f64, t3837: f64, t1365: f64, t68: f64, t3734: f64, t1347: f64, t3719: f64, t1345: f64, t1348: f64, t546: f64, t548: f64) -> (f64, f64, f64, f64) {
    let t3839 = (t3817 + t3837) * t225;
    let t3843 = t68 * t1365;
    let t3844 = t3843 * t3734;
    let t3847 = t1347 * t3719;
    let t3850 = 6.0_f64 * t1345 * t1348 - t3839 * t548 - 12.0_f64 * t3844 * t546 + 3.0_f64 * t3847 * t546;
    (t3839, t3844, t3847, t3850)
}
