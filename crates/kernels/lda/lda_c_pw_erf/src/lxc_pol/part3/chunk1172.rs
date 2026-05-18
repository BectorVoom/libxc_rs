//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1172/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1172<F: Float>(t13115: F, t13117: F, t5160: F, t10027: F, t5167: F, t10438: F, t13764: F, t13765: F, t13766: F, t13768: F, t13769: F, t13770: F, t13776: F, t13781: F, t13784: F, t13787: F) -> (F, F, F) {
    let t13790 = F::new(64.0) / F::new(15.0) * t13115 * t5160 * t13117;
    let t13792 = F::new(16.0) / F::new(9.0) * t10027 * t5167;
    let t13793 = -t13764 + t13765 + t13766 - t13768 - t13769 - t10438 - t13770 + t13776 - t13781 - t13784 - t13787 + t13790 + t13792;
    (t13790, t13792, t13793)
}
