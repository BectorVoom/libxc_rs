//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 420/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk420<F: Float>(t110: F, t10: F, t107: F, t119: F, t142: F, t3020: F, t64: F, t903: F, t41: F, t120: F, t912: F, t919: F, t212: F, t9: F) -> (F, F, F, F) {
    let t111 = t110 < -F::cast_from(0.66725e-1_f64);
    let t3031 = piecewise3::<F>(t111, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t64 * t3020 * t10 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t64 * t903 * t142 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t64 * t107 * t119);
    let t3032 = t3031 * t41;
    let t3033 = t3032 * t120;
    let t3036 = t912 * t919;
    let t3042 = F::cast_from(1.0_f64) / t9 / t212;
    (t3032, t3033, t3036, t3042)
}
