//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1089/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1089<F: Float>(t101: F, t18866: F, t19866: F, t2644: F, t2775: F, t6154: F, t6155: F, t6156: F, t8771: F, t8774: F, t8793: F, t8805: F, t8808: F, t8812: F, t8821: F, t8822: F, t8825: F, t8827: F, t8831: F, t8834: F, t8838: F) -> F {
    let t20246 = F::cast_from(4.0_f64) * t101 * t2644 * t2775 * t6156 + F::cast_from(4.0_f64) * t6154 * t19866 * t6155 + F::cast_from(0.05987117005127304_f64) * t8771 + t8774 - F::cast_from(0.01197423401025461_f64) * t8793 - t8805 - F::cast_from(4.569219094474146e-06_f64) * t8808 - t8812 + t8821 - F::cast_from(5.4655730795145296e-05_f64) * t18866 + F::cast_from(0.19513566535229734_f64) * t8822 + t8825 + F::cast_from(0.004067943812504169_f64) * t8827 + t8831 - F::cast_from(0.006715335817467199_f64) * t8834 - t8838;
    t20246
}
