//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 959/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk959(t118858: f64, t1880: f64, t214: f64, t225: f64, t258: f64, t28406: f64, t118910: f64, t6552: f64, t7479: f64, t28276: f64, t30663: f64, t1484: f64, t1527: f64) -> (f64, f64, f64, f64, f64) {
    let t126399 = 0.76763589786250567036e-1_f64 * t118858;
    let t126404 = 0.16449340668482264365e-1_f64 * t1880 * t214 * t28406 * t225 * t258;
    let t126409 = 0.6579736267392905746e-1_f64 * t6552 * t118910 * t7479;
    let t126412 = 0.3289868133696452873e-1_f64 * t6552 * t30663 * t28276;
    let t126413 = t1484 * t1527;
    (t126399, t126404, t126409, t126412, t126413)
}
