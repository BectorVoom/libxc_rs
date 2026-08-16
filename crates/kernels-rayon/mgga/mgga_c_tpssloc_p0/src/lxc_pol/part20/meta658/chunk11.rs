//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2451/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2451(t3070: f64, t43198: f64, t4578: f64, t4574: f64, t14192: f64, t2960: f64, t10510: f64, t4641: f64, t10316: f64, t10481: f64, t10483: f64, t10877: f64, t10952: f64, t14099: f64, t1616: f64, t3071: f64, t42347: f64, t42511: f64, t42743: f64, t43176: f64, t43291: f64, t43292: f64, t43385: f64, t4579: f64, t4582: f64, t45872: f64, t4593: f64, t4600: f64, t973: f64, t974: f64, t998: f64) -> f64 {
    let t50147 = t3070 * t43198 * t4578;
    let t50148 = t50147 / 6912.0_f64;
    let t50169 = t3070 * t43198 * t4574;
    let t50170 = t50169 / 6912.0_f64;
    let t50172 = t2960 * t14192;
    let t50174 = t4641 * t10510;
    let t50175 = t50174 / 4608.0_f64;
    let t50176 = t973 * t974 * t998 * t45872 / 288.0_f64 + t3070 * t3071 * t1616 * t10316 / 768.0_f64 - t50148 + t42511 * t4579 / 1536.0_f64 + t43291 * t4582 * t4593 * t43292 * t10481 / 128.0_f64 - 3.0_f64 / 256.0_f64 * t43385 * t4582 * t4593 * t10483 + 7.0_f64 / 1536.0_f64 * t42347 * t4582 * t4593 * t10877 - t42743 * t4600 / 1024.0_f64 - t10952 * t14099 / 512.0_f64 - t50170 + t43176 / 4608.0_f64 - t50172 / 54.0_f64 - t50175;
    t50176
}
