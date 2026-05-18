//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 254/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk254<F: Float>(t1124: F, t1154: F, t431: F, t426: F, t1118: F, t1143: F, t1135: F, t1140: F, t1147: F, t434: F, t448: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1156 = -t1154 + F::new(0.17123333333333333333e-1) * t1124;
    let t1159 = t431 * t431;
    let t1160 = F::new(1.0) / t1159;
    let t1161 = t426 * t1160;
    let t1163 = F::new(0.516475e0) * t1118;
    let t1166 = F::new(0.104195e0) * t1143;
    let t1168 = F::new(0.3529725e1) * t1135 - t1163 + F::new(0.516475e0) * t1124 + F::new(0.6311625e0) * t1140 - t1166 + F::new(0.104195e0) * t1147;
    let t1169 = F::new(1.0) / t434;
    let t1170 = t1168 * t1169;
    let t1173 = F::new(0.92708333333333333333e-2) * t1118;
    let t1175 = -t1173 + F::new(0.92708333333333333333e-2) * t1124;
    let t1176 = t1175 * t448;
    (t1156, t1159, t1160, t1161, t1163, t1166, t1168, t1169, t1170, t1173, t1175, t1176)
}
