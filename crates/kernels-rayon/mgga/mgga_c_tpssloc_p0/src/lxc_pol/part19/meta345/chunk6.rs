//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1240/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1240(t40995: f64, t41037: f64, t41077: f64, t41120: f64, t41343: f64, t41393: f64, t41434: f64, t41487: f64, t10069: f64, t10077: f64, t10080: f64, t10091: f64, t10098: f64, t13390: f64, t13397: f64, t226: f64, t22997: f64, t235: f64, t2617: f64, t2728: f64, t2732: f64, t40926: f64, t40932: f64, t40934: f64, t40938: f64, t40951: f64, t40955: f64, t4291: f64, t812: f64, t829: f64, t9958: f64) -> (f64, f64) {
    let t41490 = t40995 + t41037 + t41077 + t41120 + t41343 + t41393 + t41434 + t41487;
    let t41495 = -36.0_f64 * t10080 * t40938 * t812 - 36.0_f64 * t13397 * t22997 * t40951 + t226 * t235 * t41490 + 14.0_f64 * t2728 * t40926 * t812 - 4.0_f64 * t2732 * t812 * t9958 + 24.0_f64 * t40932 * t40934 * t812 - 12.0_f64 * t40955 * t4291 * t829 - 4.0_f64 * t10069 * t2617 - 12.0_f64 * t10077 * t2617 - 12.0_f64 * t10091 * t2617 - 12.0_f64 * t10098 * t13390;
    (t41490, t41495)
}
