//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 988/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk988<F: Float>(t2158: F, t2170: F, t3178: F, t3138: F, t3165: F, t5: F, t3139: F, t3140: F, t2142: F, t3108: F, t3106: F, t4395: F) -> (F, F, F, F, F, F, F) {
    let t8837 = t2170 * t3178 * t2158;
    let t8839 = t3138 * t8837 / F::new(24.0);
    let t8840 = t5 * t3165;
    let t8842 = t3139 * t8840 * t3140;
    let t8844 = t3138 * t8842 / F::new(24.0);
    let t8846 = F::new(7.0) / F::new(144.0) * t3108 * t2142;
    let t8847 = t4395 * t3106;
    (t8837, t8839, t8840, t8842, t8844, t8846, t8847)
}
