//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1098/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1098<F: Float>(t5686: F, t9744: F, t221: F, t4019: F, t5659: F, t4018: F, t3989: F, t5629: F, t3930: F, t5661: F, t5665: F, t9976: F) -> (F, F, F, F, F) {
    let t14024 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t9744 * t5686;
    let t14036 = t4019 * t221 * t5659;
    let t14038 = F::cast_from(0.25410001404642664112e-4_f64) * t4018 * t14036;
    let t14040 = F::cast_from(0.40015750243531754508e-1_f64) * t3989 * t5629;
    let t14042 = F::cast_from(0.20007875121765877254e-2_f64) * t3930 * t5661;
    let t14043 = t9976 * t5665;
    (t14024, t14038, t14040, t14042, t14043)
}
