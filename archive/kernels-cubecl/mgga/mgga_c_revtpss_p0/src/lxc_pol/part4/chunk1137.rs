//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1137/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1137<F: Float>(t221: F, t4019: F, t5659: F, t4018: F, t3989: F, t5629: F, t3930: F, t5661: F, t5665: F, t9976: F, t1412: F, t1882: F) -> (F, F, F, F, F) {
    let t14036 = t4019 * t221 * t5659;
    let t14038 = F::cast_from(0.25410001404642664112e-4_f64) * t4018 * t14036;
    let t14040 = F::cast_from(0.40015750243531754508e-1_f64) * t3989 * t5629;
    let t14042 = F::cast_from(0.20007875121765877254e-2_f64) * t3930 * t5661;
    let t14043 = t9976 * t5665;
    let t14045 = t1412 * t1882;
    (t14038, t14040, t14042, t14043, t14045)
}
