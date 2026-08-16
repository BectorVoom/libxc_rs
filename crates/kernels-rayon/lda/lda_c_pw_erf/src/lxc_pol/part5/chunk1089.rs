//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1089/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1089(t101: f64, t18866: f64, t19866: f64, t2644: f64, t2775: f64, t6154: f64, t6155: f64, t6156: f64, t8771: f64, t8774: f64, t8793: f64, t8805: f64, t8808: f64, t8812: f64, t8821: f64, t8822: f64, t8825: f64, t8827: f64, t8831: f64, t8834: f64, t8838: f64) -> f64 {
    let t20246 = 4.0_f64 * t101 * t2644 * t2775 * t6156 + 4.0_f64 * t6154 * t19866 * t6155 + 0.05987117005127304_f64 * t8771 + t8774 - 0.01197423401025461_f64 * t8793 - t8805 - 4.569219094474146e-06_f64 * t8808 - t8812 + t8821 - 5.4655730795145296e-05_f64 * t18866 + 0.19513566535229734_f64 * t8822 + t8825 + 0.004067943812504169_f64 * t8827 + t8831 - 0.006715335817467199_f64 * t8834 - t8838;
    t20246
}
