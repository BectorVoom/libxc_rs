//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 843/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk843(t44492: f64, t1063: f64, t13299: f64, t13316: f64, t13323: f64, t1339: f64, t13445: f64, t2268: f64, t2765: f64, t31747: f64, t35220: f64, t419: f64, t44457: f64, t44469: f64, t44473: f64, t44477: f64, t44479: f64, t44483: f64, t44485: f64, t44487: f64, t44489: f64, t44490: f64, t44491: f64, t448: f64, t6313: f64, t7937: f64) -> f64 {
    let t44493 = 0.15808337019820083111e-2_f64 * t44492;
    let t44501 = -t44457 - 0.28455006635676149599e-1_f64 * t1063 * t13445 * t448 + 0.7588001769513639893e-1_f64 * t6313 * t13316 - 0.28455006635676149599e-1_f64 * t419 * t13323 + 0.28455006635676149599e-1_f64 * t419 * t13299 - t44469 + t44473 - t44477 + t44479 + t44483 + t44485 + t44487 - t44489 + t44490 + t44491 - t44493 - 0.39837009289946609438e0_f64 * t2268 * t2765 * t35220 + 0.68292015925622759038e0_f64 * t2268 * t7937 * t1339 * t31747;
    t44501
}
