//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1471/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1471(t1214: f64, t1227: f64, t1230: f64, t1737: f64, t19033: f64, t19051: f64, t19083: f64, t22214: f64, t22284: f64, t248: f64, t3515: f64, t3585: f64, t44836: f64, t475: f64, t5024: f64, t52766: f64, t6203: f64, t6207: f64, t6227: f64, t6232: f64, t65963: f64, t65966: f64, t72363: f64, t72936: f64, t72959: f64, t77973: f64, t77977: f64, t78757: f64, t79018: f64) -> f64 {
    let t79251 = -t72936 / 288.0_f64 + t52766 * t22284 / 384.0_f64 - t1227 * t248 * t1230 * t77977 / 192.0_f64 + t72363 * t1737 / 768.0_f64 - t44836 * t248 * t1214 * t79018 * t475 / 3072.0_f64 + t19083 * t6207 / 72.0_f64 + t5024 * t22214 / 216.0_f64 + t65963 * t6227 / 256.0_f64 - t65966 * t6232 / 512.0_f64 + 5.0_f64 / 2304.0_f64 * t19051 * t6203 + 5.0_f64 / 384.0_f64 * t1227 * t248 * t3585 * t77973 - t3515 * t248 * t1214 * t78757 * t475 / 1024.0_f64 + 95.0_f64 / 1296.0_f64 * t19033 * t6203 - t72959 / 576.0_f64;
    t79251
}
