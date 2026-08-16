//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1779/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1779<F: Float>(t12470: F, t17097: F, t17154: F, t1744: F, t20625: F, t24331: F, t24376: F, t24414: F, t24423: F, t3477: F, t3479: F, t435: F, t58262: F, t58304: F, t6502: F, t6519: F, t69371: F, t81836: F, t90505: F, t90509: F, t90511: F, t90514: F, t90578: F, t90580: F, t90582: F, t90585: F, t90629: F, t90670: F, t90836: F, t90848: F) -> F {
    let t90852 = F::cast_from(0.12865583598954028054e3_f64) * t3477 * t81836 * t1744 + F::cast_from(0.12414243100625616072e5_f64) * t12470 * t20625 * t6502 - F::cast_from(0.14035736694323150897e2_f64) * t17154 * t24423 + F::cast_from(0.20779030926817756511e3_f64) * t17097 * t24414 - F::cast_from(0.77193501593724168322e3_f64) * t58304 * t24331 + F::cast_from(0.11579025239058625248e4_f64) * t12470 * t90670 * t3479 - F::cast_from(0.70178683471615754484e1_f64) * t69371 * t6519 - F::cast_from(0.4155806185363551302e3_f64) * t58262 * t24376 - t90505 - t90509 - t90511 + t90514 - t90578 + t90580 + t90582 - t90585 - F::cast_from(0.310907e-1_f64) * (t90836 + t90848) * t435 + t90629;
    t90852
}
