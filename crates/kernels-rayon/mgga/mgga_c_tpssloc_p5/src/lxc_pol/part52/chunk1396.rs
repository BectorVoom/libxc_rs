//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1396/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1396(t2174: f64, t7758: f64, t2169: f64, t7774: f64, t116326: f64, t116328: f64, t123261: f64, t123280: f64, t123292: f64, t123304: f64, t123313: f64, t1398: f64, t1404: f64, t1852: f64, t2023: f64, t2029: f64, t2170: f64, t26510: f64, t26555: f64, t27908: f64, t27930: f64, t3: f64, t31949: f64, t33762: f64, t5364: f64, t5381: f64, t580: f64, t7003: f64, t7426: f64, t7759: f64, t8119: f64, t8693: f64, t8702: f64) -> f64 {
    let t123319 = t7758 * t2174;
    let t123322 = t2169 * t7774;
    let t123325 = t3 * t123261 * t580 + t2170 * t26555 + t7759 * t7426 + t116328 + t1852 * t31949 + t7003 * t8119 + t116326 + t1398 * (t123280 + t123292 + t123304 + t123313) + t2023 * t27930 + t26510 * t2174 + t123319 + t33762 * t1404 + t27908 * t2029 + t123322 + t8693 * t5381 + t5364 * t8702;
    t123325
}
