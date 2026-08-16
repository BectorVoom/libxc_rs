//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 870/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk870(t143: f64, t169: f64, t2592: f64, t2645: f64, t2676: f64, t2785: f64, t281: f64, t2822: f64, t2828: f64, t2835: f64, t2841: f64, t2847: f64, t2876: f64, t299: f64, t301: f64, t4427: f64, t4435: f64, t4439: f64, t4449: f64, t4455: f64, t4457: f64, t6094: f64, t7339: f64, t7387: f64, t777: f64, t7984: f64, t7988: f64, t7992: f64, t7996: f64, t8074: f64) -> f64 {
    let t8076 = -0.01197423401025461_f64 * t281 * t7339 - 0.0017434044910732151_f64 * t4427 + 0.5945049527603057_f64 * t4435 + 0.004067943812504169_f64 * t4439 - 2.0_f64 * t2645 * t2676 + 2.0_f64 * t2645 * t2592 + 0.020267214298646783_f64 * t169 * t299 * t7387 * t301 + 0.11974234010254609_f64 * t4455 - 0.15965645347006147_f64 * t4457 + t2785 + t7984 + 6.0_f64 * t7996 * t143 + 18.0_f64 * t4449 * t6094 + 2.0_f64 * t777 * t7988 - t777 * t7992 - t2822 + t2828 - t2835 - t2841 + t2847 - t2876 + t8074;
    t8076
}
