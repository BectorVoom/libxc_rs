//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 911/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk911(t21444: f64, t340: f64, t343: f64, t974: f64, t1597: f64, t5836: f64, t4546: f64, t5842: f64, t20217: f64, t978: f64, t977: f64, t10217: f64, t20234: f64) -> (f64, f64, f64, f64, f64) {
    let t21446 = t340 * t21444 * t343;
    let t21447 = t974 * t21446;
    let t21452 = t5836 * t1597 * t343;
    let t21453 = t4546 * t21452;
    let t21456 = t5842 * t1597;
    let t21458 = t340 * t21456 * t343;
    let t21459 = t974 * t21458;
    let t21462 = t978 * t20217;
    let t21463 = t977 * t21462;
    let t21468 = t10217 * t20234;
    (t21447, t21453, t21459, t21463, t21468)
}
