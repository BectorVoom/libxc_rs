//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1132/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1132(t10907: f64, t2201: f64, t3602: f64, t11824: f64, t2207: f64, t3336: f64, t10781: f64, t7970: f64, t2553: f64, t37764: f64, t11693: f64, t6205: f64) -> (f64, f64, f64, f64, f64) {
    let t39569 = t2201 * t10907 * t3602;
    let t39572 = t2207 * t3336 * t11824;
    let t39577 = t10781 * t7970;
    let t39579 = t37764 * t2553;
    let t39580 = 0.25610080155860322884e0_f64 * t39579;
    let t39581 = t6205 * t11693;
    (t39569, t39572, t39577, t39580, t39581)
}
