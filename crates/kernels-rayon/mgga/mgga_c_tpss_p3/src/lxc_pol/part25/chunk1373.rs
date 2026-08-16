//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1373/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1373(t1006: f64, t1497: f64, t1692: f64, t1812: f64, t18728: f64, t20025: f64, t20054: f64, t20417: f64, t20510: f64, t20514: f64, t20526: f64, t21659: f64, t2439: f64, t5853: f64, t6354: f64, t70800: f64, t70803: f64, t70844: f64, t70850: f64, t70854: f64, t70868: f64, t70906: f64, t70929: f64, t70932: f64, t72298: f64, t72310: f64) -> f64 {
    let t72561 = -6.0_f64 * t20417 * t70800 + 6.0_f64 * t18728 * t70854 + 6.0_f64 * t20417 * t70906 + t1692 * t20510 * t1497 + 3.0_f64 * t2439 * t1812 * t70868 - t1692 * t20514 * t20054 - 3.0_f64 * t18728 * t70932 + 3.0_f64 * t20417 * t70929 - t1692 * t5853 * t70850 / 2.0_f64 + 3.0_f64 * t2439 * t6354 * t20025 + t1692 * t21659 * t1006 / 2.0_f64 - t72298 + 6.0_f64 * t20417 * t70844 + t20526 * t70803 - t72310;
    t72561
}
