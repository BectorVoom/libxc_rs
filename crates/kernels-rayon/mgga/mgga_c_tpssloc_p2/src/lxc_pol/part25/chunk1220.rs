//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1220/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1220(t10121: f64, t10140: f64, t13487: f64, t1877: f64, t193: f64, t202: f64, t2056: f64, t2057: f64, t2379: f64, t24335: f64, t24339: f64, t24344: f64, t2522: f64, t2553: f64, t2745: f64, t2749: f64, t4314: f64, t46240: f64, t46252: f64, t46298: f64, t46320: f64, t46362: f64, t7110: f64, t7114: f64, t776: f64, t84766: f64, t84791: f64, t84800: f64, t85166: f64, t868: f64, t870: f64, t9458: f64, t9516: f64, t9616: f64) -> f64 {
    let t85243 = -18.0_f64 * t2522 * t24339 * t13487 + 3.0_f64 * t2522 * t2057 * t9516 + t193 * t202 * t85166 * t870 - 9.0_f64 * t2522 * t7114 * t46252 - 9.0_f64 * t2522 * t7114 * t46240 + 18.0_f64 * t2522 * t24344 * t46320 - 3.0_f64 * t1877 * t24339 * t2745 + 9.0_f64 * t2522 * t7110 * t2553 - 18.0_f64 * t4314 * t7114 * t46298 + 18.0_f64 * t4314 * t2057 * t9616 + 6.0_f64 * t193 * t9458 * t2056 * t870 + 18.0_f64 * t4314 * t7110 * t2379 + 6.0_f64 * t1877 * t24344 * t46362 + 9.0_f64 * t2522 * t24335 * t776 - 6.0_f64 * t1877 * t84766 * t10140 + 6.0_f64 * t1877 * t84800 * t2749 - t1877 * t7114 * t10121 - 3.0_f64 * t1877 * t84791 * t868;
    t85243
}
