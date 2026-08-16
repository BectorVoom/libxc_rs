//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1293/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1293(t1842: f64, t3911: f64, t3887: f64, t3888: f64, t12021: f64, t12033: f64, t1375: f64, t1386: f64, t16453: f64, t16458: f64, t16460: f64, t16463: f64, t16465: f64, t16468: f64, t1843: f64, t3758: f64, t3882: f64, t3889: f64, t5215: f64, t5326: f64, t5354: f64, t568: f64) -> f64 {
    let t16470 = t1842 * t3911;
    let t16471 = t3887 * t16470;
    let t16474 = t1842 * t3888;
    let t16475 = t12021 * t16474;
    let t16485 = -t12033 * t1843 + 4.0_f64 * t1375 * t16453 + 2.0_f64 * t1375 * t16471 - 6.0_f64 * t1375 * t16475 - 2.0_f64 * t1386 * t16460 + t16458 * t568 + t16463 * t568 + 2.0_f64 * t16465 * t568 + t16468 * t568 + 4.0_f64 * t3758 * t5326 + 4.0_f64 * t3882 * t5326 - 2.0_f64 * t3882 * t5354 + 2.0_f64 * t3889 * t5215;
    t16485
}
