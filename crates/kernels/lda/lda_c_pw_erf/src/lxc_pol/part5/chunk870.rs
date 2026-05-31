//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 870/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk870<F: Float>(t143: F, t169: F, t2592: F, t2645: F, t2676: F, t2785: F, t281: F, t2822: F, t2828: F, t2835: F, t2841: F, t2847: F, t2876: F, t299: F, t301: F, t4427: F, t4435: F, t4439: F, t4449: F, t4455: F, t4457: F, t6094: F, t7339: F, t7387: F, t777: F, t7984: F, t7988: F, t7992: F, t7996: F, t8074: F) -> F {
    let t8076 = -F::cast_from(0.01197423401025461_f64) * t281 * t7339 - F::cast_from(0.0017434044910732151_f64) * t4427 + F::cast_from(0.5945049527603057_f64) * t4435 + F::cast_from(0.004067943812504169_f64) * t4439 - F::cast_from(2.0_f64) * t2645 * t2676 + F::cast_from(2.0_f64) * t2645 * t2592 + F::cast_from(0.020267214298646783_f64) * t169 * t299 * t7387 * t301 + F::cast_from(0.11974234010254609_f64) * t4455 - F::cast_from(0.15965645347006147_f64) * t4457 + t2785 + t7984 + F::cast_from(6.0_f64) * t7996 * t143 + F::cast_from(18.0_f64) * t4449 * t6094 + F::cast_from(2.0_f64) * t777 * t7988 - t777 * t7992 - t2822 + t2828 - t2835 - t2841 + t2847 - t2876 + t8074;
    t8076
}
