//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2537/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2537(t1147: f64, t14933: f64, t3400: f64, t4832: f64, t11282: f64, t1687: f64, t1129: f64, t11311: f64, t1137: f64, t11400: f64, t11410: f64, t1157: f64, t15118: f64, t15121: f64, t1695: f64, t3327: f64, t3396: f64, t3404: f64, t44183: f64, t4820: f64, t4835: f64, t50821: f64, t51119: f64, t51122: f64, t51124: f64, t51126: f64, t51128: f64, t51267: f64, t51279: f64, t51293: f64, t51306: f64, t51320: f64, t51332: f64, t51346: f64, t51359: f64) -> f64 {
    let t51366 = t14933 * t1147;
    let t51371 = t4832 * t3400;
    let t51376 = t1687 * t11282;
    let t51381 = t50821 - t51119 - t51122 - t51124 - t51126 - t51128 + 3.0_f64 * t11410 * t4820 + 3.0_f64 * t3327 * t15118 + 1.0_f64 * t1129 * (t51267 + t51279 + t51293 + t51306 + t51320 + t51332 + t51346 + t51359) * t1137 + 0.17544670867903938621e1_f64 * t51366 * t1157 + 0.17544670867903938621e1_f64 * t15121 * t3396 + 0.51947577317044391276e2_f64 * t51371 * t3404 + 0.5848223622634646207e0_f64 * t4835 * t11400 + 0.10254018858216406658e4_f64 * t51376 * t11311 + 0.5848223622634646207e0_f64 * t44183 * t1695;
    t51381
}
