//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 846/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk846<F: Float>(t40283: F, t1445: F, t1562: F, t41784: F, t12881: F, t9497: F, t8248: F, t9565: F, t40301: F, t41809: F, t6508: F, t4820: F, t6824: F) -> (F, F, F, F, F, F, F) {
    let t41984 = F::cast_from(0.59584149919750711116e-1_f64) * t40283;
    let t41987 = F::cast_from(0.62115540045351614476e2_f64) * t1562 * t1445 * t41784;
    let t41989 = F::cast_from(0.25025342966295298669e1_f64) * t9497 * t12881;
    let t41991 = F::cast_from(0.11916829983950142223e0_f64) * t8248 * t9565;
    let t41992 = F::cast_from(0.38342925953920749676e1_f64) * t40301;
    let t41993 = t6508 * t41809;
    let t41996 = F::cast_from(0.79445533226334281487e-1_f64) * t6824 * t4820 * t41993;
    (t41984, t41987, t41989, t41991, t41992, t41993, t41996)
}
