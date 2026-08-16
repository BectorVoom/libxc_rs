//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2044/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2044(t2221: f64, t3826: f64, t12132: f64, t592: f64, t1336: f64, t1339: f64, t2691: f64, t12344: f64, t3777: f64, t10021: f64, t154: f64, t59: f64) -> (f64, f64, f64, f64, f64) {
    let t40225 = t2221 * t3826;
    let t40230 = 16.0_f64 * t592 * t12132;
    let t40281 = t1336 * t1339 * t2691;
    let t40292 = t3777 * t12344;
    let t40341 = t59 * t10021 * t154;
    (t40225, t40230, t40281, t40292, t40341)
}
