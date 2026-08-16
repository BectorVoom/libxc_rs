//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1057/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1057(t10353: f64, t929: f64, t926: f64, t11493: f64, t11497: f64, t11501: f64, t11508: f64, t11509: f64, t2685: f64, t2731: f64, t3924: f64, t3935: f64, t8577: f64, t8588: f64, t8954: f64, t8966: f64, t8972: f64, t925: f64) -> f64 {
    let t11512 = t929 * t10353;
    let t11513 = t926 * t11512;
    let t11518 = t8588 / 81.0_f64 - t8954 / 10368.0_f64 - t8966 / 432.0_f64 - t2731 * t11493 / 1536.0_f64 - t2731 * t11497 / 3072.0_f64 + t8577 * t11501 / 3072.0_f64 - t8972 * t3935 / 144.0_f64 + t11508 + t925 * t11509 / 48.0_f64 + t925 * t11513 / 288.0_f64 + t2685 * t3924 / 27.0_f64;
    t11518
}
