//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1310/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1310(t4300: f64, t865: f64, t2718: f64, t2684: f64, t4180: f64, t4181: f64, t9646: f64, t9647: f64, t2633: f64, t2645: f64, t4248: f64, t1496: f64, t9541: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13071 = t4300 * t865;
    let t13072 = t2718 * t13071;
    let t13076 = t4180 * t4181 * t2684;
    let t13080 = t9646 * t4181 * t9647;
    let t13084 = t2645 * t4248 * t2633;
    let t13087 = t9541 * t1496;
    (t13071, t13072, t13076, t13080, t13084, t13087)
}
