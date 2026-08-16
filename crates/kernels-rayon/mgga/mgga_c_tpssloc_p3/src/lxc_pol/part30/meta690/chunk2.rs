//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2201/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2201(t1877: f64, t1915: f64, t22959: f64, t25013: f64, t25024: f64, t2522: f64, t25354: f64, t25358: f64, t25377: f64, t25392: f64, t28241: f64, t28242: f64, t28252: f64, t28256: f64, t28456: f64, t4314: f64, t46341: f64, t6666: f64, t7475: f64, t7541: f64, t81539: f64, t97950: f64, t97953: f64, t97956: f64, t97972: f64, t97985: f64) -> f64 {
    let t97989 = 6.0_f64 * t25013 * t97950 - 3.0_f64 * t22959 * t97953 - 3.0_f64 * t25013 * t97956 + 3.0_f64 * t2522 * t7541 * t25024 - t1877 * t25358 * t25392 + 3.0_f64 * t4314 * t6666 * t28241 + 3.0_f64 * t2522 * t25354 * t7475 + t97972 + t1877 * t81539 * t28456 - t1877 * t25358 * t25377 + 3.0_f64 / 2.0_f64 * t2522 * t6666 * t28256 + 3.0_f64 * t46341 * t28242 + 3.0_f64 * t2522 * t6666 * t28252 + 3.0_f64 * t4314 * t1915 * t97985;
    t97989
}
