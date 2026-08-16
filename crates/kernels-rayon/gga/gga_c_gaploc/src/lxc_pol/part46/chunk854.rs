//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 854/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk854(t40252: f64, t40258: f64, t40261: f64, t40277: f64, t40280: f64, t40283: f64, t1445: f64, t1562: f64, t41784: f64, t12881: f64, t9497: f64, t8248: f64, t9565: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41979 = 0.29792074959875355558e-1_f64 * t40252;
    let t41980 = 0.20854452471912748891e0_f64 * t40258;
    let t41981 = 0.12780975317973583225e0_f64 * t40261;
    let t41982 = 0.17875244975925213335e0_f64 * t40277;
    let t41983 = 0.11916829983950142223e0_f64 * t40280;
    let t41984 = 0.59584149919750711116e-1_f64 * t40283;
    let t41987 = 0.62115540045351614476e2_f64 * t1562 * t1445 * t41784;
    let t41989 = 0.25025342966295298669e1_f64 * t9497 * t12881;
    let t41991 = 0.11916829983950142223e0_f64 * t8248 * t9565;
    (t41979, t41980, t41981, t41982, t41983, t41984, t41987, t41989, t41991)
}
