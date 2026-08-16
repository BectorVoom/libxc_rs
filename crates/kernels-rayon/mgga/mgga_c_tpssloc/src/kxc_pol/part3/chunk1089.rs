//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1089/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1089(t14164: f64, t14165: f64, t4582: f64, t10263: f64, t10403: f64, t1041: f64, t10413: f64, t10896: f64, t14122: f64, t14126: f64, t14130: f64, t14136: f64, t14139: f64, t14143: f64, t14147: f64, t14152: f64, t14158: f64, t14160: f64, t1607: f64, t2960: f64, t3070: f64, t3117: f64, t4562: f64, t4565: f64, t4585: f64, t973: f64) -> f64 {
    let t14166 = t14164 * t14165;
    let t14167 = t4582 * t14166;
    let t14170 = t10403 * t14122 / 2304.0_f64 - t10413 * t14126 / 4608.0_f64 - t3070 * t14130 / 2304.0_f64 - t10896 / 4608.0_f64 - t14136 + t14139 - t3117 * t4585 / 1152.0_f64 - t1041 * t14143 / 1152.0_f64 - t1041 * t14147 / 2304.0_f64 + t2960 * t4562 / 27.0_f64 + t973 * t14152 / 48.0_f64 - 2.0_f64 / 81.0_f64 * t2960 * t4565 - t14158 - t14160 / 1296.0_f64 + 11.0_f64 / 324.0_f64 * t10263 * t1607 + t1041 * t14167 / 768.0_f64;
    t14170
}
