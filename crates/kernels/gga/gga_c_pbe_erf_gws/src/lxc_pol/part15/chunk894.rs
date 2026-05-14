//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 894/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk894<F: Float>(t2171: F, t8827: F, t4386: F, t2168: F, t6185: F, t3179: F, t6331: F, t2146: F, t2158: F, t2170: F, t3178: F, t3138: F, t3165: F, t5: F, t3139: F, t3140: F) -> (F, F, F, F, F, F, F, F) {
    let t8828 = t8827 * t2171;
    let t8829 = t4386 * t8828;
    let t8831 = t2168 * t8829 / 24.0;
    let t8832 = 7.0 / 144.0 * t6185;
    let t8833 = t6331 * t3179;
    let t8835 = 7.0 / 72.0 * t2146 * t8833;
    let t8837 = t2170 * t3178 * t2158;
    let t8839 = t3138 * t8837 / 24.0;
    let t8840 = t5 * t3165;
    let t8842 = t3139 * t8840 * t3140;
    (t8828, t8831, t8832, t8835, t8837, t8839, t8840, t8842)
}
