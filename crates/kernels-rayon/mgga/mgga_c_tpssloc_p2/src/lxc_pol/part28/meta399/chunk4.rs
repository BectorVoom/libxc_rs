//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1552/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1552(t16131: f64, t16435: f64, t1378: f64, t225: f64, t5319: f64, t1372: f64, t5210: f64, t12030: f64, t12444: f64, t1375: f64, t1386: f64, t16022: f64, t16028: f64, t16030: f64, t1843: f64, t3758: f64, t3889: f64, t3912: f64, t5215: f64, t5321: f64, t5354: f64, t568: f64) -> (f64, f64, f64, f64, f64) {
    let t16436 = t16131 + t16435;
    let t16437 = t1378 * t16436;
    let t16439 = t5319 * t225;
    let t16448 = t5210 * t1372;
    let t16451 = -t12030 * t1843 - 2.0_f64 * t12444 * t1843 - t1375 * t16437 - 2.0_f64 * t1386 * t16022 - 2.0_f64 * t1386 * t16030 - 2.0_f64 * t1386 * t16439 + t16028 * t568 + 2.0_f64 * t16448 * t568 - 2.0_f64 * t3758 * t5354 + 2.0_f64 * t3889 * t5321 - t3912 * t5215 - t3912 * t5321;
    (t16436, t16437, t16439, t16448, t16451)
}
