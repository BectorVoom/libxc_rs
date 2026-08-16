//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1151/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1151(t12442: f64, t225: f64, t12036: f64, t12016: f64, t12440: f64, t3911: f64, t12021: f64, t12027: f64, t12030: f64, t12033: f64, t12437: f64, t12438: f64, t12444: f64, t1375: f64, t1385: f64, t1386: f64, t3758: f64, t3887: f64, t3888: f64, t3889: f64, t3912: f64) -> f64 {
    let t39910 = t12442 * t225;
    let t39913 = t12036 * t225;
    let t39916 = t12016 * t225;
    let t39919 = t12440 * t225;
    let t39922 = t3911 * t3911;
    let t39932 = -36.0_f64 * t12021 * t1375 * t3888 * t3911 + 8.0_f64 * t12437 * t1375 * t1385 * t3887 + 6.0_f64 * t1375 * t3887 * t39922 + 24.0_f64 * t12027 * t3758 - 6.0_f64 * t12030 * t3912 - 6.0_f64 * t12033 * t3912 - 4.0_f64 * t12438 * t3758 + 24.0_f64 * t12444 * t3889 - 12.0_f64 * t12444 * t3912 - 4.0_f64 * t1386 * t39910 - 12.0_f64 * t1386 * t39913 - 12.0_f64 * t1386 * t39916 - 4.0_f64 * t1386 * t39919;
    t39932
}
