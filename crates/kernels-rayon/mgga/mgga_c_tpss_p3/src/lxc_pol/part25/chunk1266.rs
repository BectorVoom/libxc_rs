//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1266/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1266(t21877: f64, t21946: f64, t3: f64, t1799: f64, t4637: f64, t1338: f64, t20690: f64, t4674: f64, t5953: f64, t117: f64, t21907: f64, t1668: f64, t1670: f64, t1851: f64, t1853: f64, t547: f64, t5470: f64, t5474: f64, t5477: f64, t548: f64, t6446: f64, t6452: f64, t6455: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21947 = t21877 + t21946;
    let t21948 = t3 * t21947;
    let t21958 = param_d * t21947;
    let t21972 = t4637 * t1799;
    let t21975 = t20690 * t1338;
    let t21978 = t5953 * t4674;
    let t21981 = t117 * t21907;
    let t21984 = 12.0_f64 * t1668 * t6452 + 6.0_f64 * t1668 * t6455 + 6.0_f64 * t1670 * t6446 + 6.0_f64 * t1851 * t5474 + 3.0_f64 * t1851 * t5477 + 3.0_f64 * t1853 * t5470 + t21958 * t548 + 6.0_f64 * t21972 * t547 + 12.0_f64 * t21975 * t547 + 6.0_f64 * t21978 * t547 + 3.0_f64 * t21981 * t547;
    (t21947, t21948, t21958, t21972, t21975, t21978, t21981, t21984)
}
