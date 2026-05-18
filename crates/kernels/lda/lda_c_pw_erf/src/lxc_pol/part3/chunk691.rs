//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 691/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk691<F: Float>(t163: F, t169: F, t616: F, t717: F, t171: F, t4150: F, t4153: F, t4156: F, t4160: F, t4163: F, t4165: F, t4168: F, t4172: F, t4175: F, t4239: F, t4246: F) -> (F, F) {
    let t4250 = t169 * t717 * t616 * t163;
    let t4252 = -F::new(0.005926167098672845) * t4150 - F::new(0.01185233419734569) * t4153 - F::new(0.0014862827083471494) * t4156 - t4160 - t4163 - t4165 + F::new(0.01975389032890948) * t4168 + t4172 + t4175 - F::new(0.005388405304614574) * t169 * t171 * t4239 * t163 + F::new(0.02694202652307287) * t4246 - F::new(0.07184540406152766) * t4250;
    (t4250, t4252)
}
