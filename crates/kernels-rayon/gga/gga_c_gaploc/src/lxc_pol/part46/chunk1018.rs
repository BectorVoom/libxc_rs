//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1018/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1018(t41466: f64, t41469: f64, t41474: f64, t41477: f64, t13019: f64, t2103: f64, t4673: f64, t11807: f64, t3277: f64, t10057: f64, t13045: f64, t11004: f64, t1445: f64, t2087: f64, t2530: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44154 = 0.29792074959875355558e-1_f64 * t41466;
    let t44155 = 0.29792074959875355558e-1_f64 * t41469;
    let t44156 = 0.25561950635947166451e1_f64 * t41474;
    let t44157 = 0.12780975317973583225e0_f64 * t41477;
    let t44159 = t2103 * t4673 * t13019;
    let t44162 = 0.25025342966295298669e1_f64 * t3277 * t11807;
    let t44164 = 0.25025342966295298669e1_f64 * t10057 * t13045;
    let t44167 = t2087 * t1445 * t11004 * t2530;
    (t44154, t44155, t44156, t44157, t44159, t44162, t44164, t44167)
}
