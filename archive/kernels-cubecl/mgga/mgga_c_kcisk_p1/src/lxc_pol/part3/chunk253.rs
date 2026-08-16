//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 253/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk253<F: Float>(t1163: F, t1186: F, t26: F, t1165: F, t1176: F, t1178: F, t1181: F, t1185: F, t320: F) -> (F, F, F, F) {
    let t1187 = t1186 * t1163;
    let t1188 = t26 * t1187;
    let t1190 = F::cast_from(0.1898925e1_f64) * t1176 - t1178 - F::cast_from(0.29896666666666666667e0_f64) * t1165 + F::cast_from(0.3071625e0_f64) * t1181 - t1185 - F::cast_from(0.82156666666666666667e-1_f64) * t1188;
    let t1191 = F::cast_from(1.0_f64) / t320;
    (t1187, t1188, t1190, t1191)
}
