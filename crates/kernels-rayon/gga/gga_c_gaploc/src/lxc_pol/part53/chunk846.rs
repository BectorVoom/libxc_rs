//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 846/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk846(t40283: f64, t1445: f64, t1562: f64, t41784: f64, t12881: f64, t9497: f64, t8248: f64, t9565: f64, t40301: f64, t41809: f64, t6508: f64, t4820: f64, t6824: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41984 = 0.59584149919750711116e-1_f64 * t40283;
    let t41987 = 0.62115540045351614476e2_f64 * t1562 * t1445 * t41784;
    let t41989 = 0.25025342966295298669e1_f64 * t9497 * t12881;
    let t41991 = 0.11916829983950142223e0_f64 * t8248 * t9565;
    let t41992 = 0.38342925953920749676e1_f64 * t40301;
    let t41993 = t6508 * t41809;
    let t41996 = 0.79445533226334281487e-1_f64 * t6824 * t4820 * t41993;
    (t41984, t41987, t41989, t41991, t41992, t41993, t41996)
}
