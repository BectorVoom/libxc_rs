//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 786/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk786(t240: f64, t4753: f64, t10549: f64, t10683: f64, t10707: f64, t10709: f64, t10712: f64, t10718: f64, t10752: f64, t10760: f64, t10773: f64, t1686: f64, t1987: f64, t4783: f64, t4791: f64, t5423: f64) -> f64 {
    let t12131 = t240 * t4753;
    let t12142 = -0.58482233974552040708e0_f64 * t1987 * t10683 - 0.17544670192365612213e1_f64 * t12131 * t1686 - 0.17544670192365612213e1_f64 * t5423 * t4783 - 0.51947267698127589899e2_f64 * t5423 * t4791 - 0.51947267698127589897e2_f64 * t1987 * t10549 + 0.19751789702565206229e-1_f64 * t240 * t10773 + t10707 + t10709 + t10712 - t10718 + t10752 + t10760;
    t12142
}
