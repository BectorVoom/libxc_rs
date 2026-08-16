//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1179/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1179(t1137: f64, t6310: f64, t1140: f64, t6314: f64, t3382: f64, t5991: f64, t1165: f64, t1173: f64, t1181: f64, t16546: f64, t16551: f64, t1889: f64, t3169: f64, t3282: f64, t335: f64, t360: f64, t3616: f64, t367: f64, t372: f64, t4450: f64, t5852: f64, t5853: f64, t5922: f64, t6309: f64, t6313: f64, t6319: f64, t6375: f64) -> f64 {
    let t21390 = t1137 * t6310;
    let t21401 = t1140 * t6314;
    let t21414 = t3382 * t5991;
    let t21421 = -t335 * t3282 * t6309 / 12.0_f64 + 7.0_f64 / 36.0_f64 * t21390 - t367 * t3282 * t6313 / 8.0_f64 + t367 * t3282 * t6319 / 12.0_f64 - t3616 * t3282 * t6375 / 2.0_f64 + 7.0_f64 / 24.0_f64 * t21401 + 0.34299214494455789578e-1_f64 * t16546 - 0.51448821741683684367e-2_f64 * t4450 * t1181 * t5852 * t5853 * t360 + 0.51448821741683684367e-2_f64 * t4450 * t1165 * t5922 * t5853 * t372 + 0.34299214494455789578e-2_f64 * t21414 + 0.34299214494455789578e-2_f64 * t1173 * t1181 * t1889 * t3169 + 0.42874018118069736972e-3_f64 * t16551;
    t21421
}
