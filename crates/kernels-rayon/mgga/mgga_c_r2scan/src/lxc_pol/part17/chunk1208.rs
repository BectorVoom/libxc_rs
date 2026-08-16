//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1208/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1208(t3275: f64, t3582: f64, t42331: f64, t44014: f64, t44017: f64, t44020: f64, t44024: f64, t44027: f64, t44029: f64, t44032: f64, t44035: f64, t44037: f64, t44039: f64, t44043: f64, t44046: f64, t44049: f64, t44051: f64, t44054: f64) -> (f64, f64) {
    let t44057 = 5.0_f64 / 8.0_f64 * t3275 * t42331 * t3582;
    let t44058 = -t44014 - t44017 + t44020 + t44024 + t44027 + t44029 + t44032 - t44035 + t44037 - t44039 - t44043 + t44046 + t44049 - t44051 - t44054 - t44057;
    (t44057, t44058)
}
