//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1779/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1779(t12470: f64, t17097: f64, t17154: f64, t1744: f64, t20625: f64, t24331: f64, t24376: f64, t24414: f64, t24423: f64, t3477: f64, t3479: f64, t435: f64, t58262: f64, t58304: f64, t6502: f64, t6519: f64, t69371: f64, t81836: f64, t90505: f64, t90509: f64, t90511: f64, t90514: f64, t90578: f64, t90580: f64, t90582: f64, t90585: f64, t90629: f64, t90670: f64, t90836: f64, t90848: f64) -> f64 {
    let t90852 = 0.12865583598954028054e3_f64 * t3477 * t81836 * t1744 + 0.12414243100625616072e5_f64 * t12470 * t20625 * t6502 - 0.14035736694323150897e2_f64 * t17154 * t24423 + 0.20779030926817756511e3_f64 * t17097 * t24414 - 0.77193501593724168322e3_f64 * t58304 * t24331 + 0.11579025239058625248e4_f64 * t12470 * t90670 * t3479 - 0.70178683471615754484e1_f64 * t69371 * t6519 - 0.4155806185363551302e3_f64 * t58262 * t24376 - t90505 - t90509 - t90511 + t90514 - t90578 + t90580 + t90582 - t90585 - 0.310907e-1_f64 * (t90836 + t90848) * t435 + t90629;
    t90852
}
