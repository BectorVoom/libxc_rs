//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2370/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2370(t2022: f64, t5381: f64, t26509: f64, t580: f64, t1404: f64, t7758: f64, t1395: f64, t7774: f64, t1396: f64, t1398: f64, t26510: f64, t26555: f64, t3: f64, t3932: f64, t5364: f64, t7020: f64, t80599: f64, t80601: f64, t80605: f64, t86640: f64, t91792: f64, t91806: f64) -> f64 {
    let t91813 = 2.0_f64 * t2022 * t5381;
    let t91816 = 2.0_f64 * t26509 * t580;
    let t91818 = 2.0_f64 * t7758 * t1404;
    let t91824 = 2.0_f64 * t1395 * t7774;
    let t91827 = t1398 * (t86640 + t91806) + t80605 + 2.0_f64 * t1396 * t26555 + t3932 * t7774 + t91813 + 2.0_f64 * t80599 + t91816 + t91818 + 2.0_f64 * t5364 * t7020 + 2.0_f64 * t26510 * t1404 + t91824 + t3 * t91792 * t580 + t80601;
    t91827
}
