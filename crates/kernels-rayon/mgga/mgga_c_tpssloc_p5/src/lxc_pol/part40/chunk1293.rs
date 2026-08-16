//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1293/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1293(t29895: f64, t30414: f64, t29900: f64, t30417: f64, t30420: f64, t110075: f64, t30407: f64, t110093: f64, t110141: f64, t110144: f64, t110526: f64, t110531: f64, t110533: f64, t110542: f64, t110564: f64, t110566: f64, t110586: f64, t110615: f64, t1453: f64, t19503: f64, t19529: f64, t2341: f64, t29903: f64, t29907: f64, t29922: f64, t30171: f64, t5396: f64, t5464: f64, t5468: f64, t5488: f64, t8128: f64, t8129: f64, t8137: f64, t8138: f64) -> f64 {
    let t111385 = t29895 * t30414;
    let t111390 = t29900 * t30417;
    let t111395 = t29900 * t30420;
    let t111408 = t110075 * t30407;
    let t111413 = 5.0_f64 / 9.0_f64 * t110526 * t2341 * t1453 * t30171 - t110531 + 10.0_f64 / 9.0_f64 * t110533 - t110542 - 2.0_f64 / 3.0_f64 * t111385 - 5.0_f64 / 12.0_f64 * t8128 * t29907 * t5488 + 10.0_f64 / 27.0_f64 * t111390 + 25.0_f64 / 108.0_f64 * t8137 * t110093 * t5468 + 5.0_f64 / 9.0_f64 * t111395 + 25.0_f64 / 72.0_f64 * t8137 * t29922 * t5396 + 22.0_f64 / 9.0_f64 * t110141 - 55.0_f64 / 27.0_f64 * t110144 + t110564 - t110566 - t110586 + t8128 * t8129 * t19529 / 4.0_f64 - 5.0_f64 / 24.0_f64 * t8137 * t8138 * t19503 + 2.0_f64 * t111408 + 5.0_f64 / 4.0_f64 * t29903 * t29907 * t5464 + t110615;
    t111413
}
