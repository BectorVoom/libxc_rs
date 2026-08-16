//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 779/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk779(t4661: f64, t654: f64, t4665: f64, t4668: f64, t669: f64, t106: f64, t1281: f64, t4675: f64, t6976: f64, t2144: f64, t4699: f64, t2182: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13277 = t654 * t4661;
    let t13279 = t654 * t4665;
    let t13300 = t4668 * t669;
    let t13307 = t106 * t1281;
    let t13316 = t6976 * t4675;
    let t13330 = t2144 * t4699;
    let t13364 = t2182 * t4665;
    (t13277, t13279, t13300, t13307, t13316, t13330, t13364)
}
