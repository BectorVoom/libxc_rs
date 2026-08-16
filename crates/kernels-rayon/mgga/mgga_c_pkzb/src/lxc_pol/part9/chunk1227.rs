//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1227/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1227(t17724: f64, t2873: f64, t730: f64, t1987: f64, t7536: f64, t1999: f64, t7560: f64, t2860: f64, t5494: f64, t307: f64, t6000: f64, t2887: f64, t2890: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21324 = 0.17315859105681463759e2_f64 * t730 * t2873 * t17724;
    let t21329 = 0.10389515463408878255e3_f64 * t1987 * t7536;
    let t21331 = 0.51947577317044391276e2_f64 * t7560 * t1999;
    let t21333 = 0.10254018858216406658e4_f64 * t2860 * t5494;
    let t21346 = t307 * t6000;
    let t21359 = t2887 * t487 * t2890;
    (t21324, t21329, t21331, t21333, t21346, t21359)
}
