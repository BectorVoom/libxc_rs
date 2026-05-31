//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 801/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk801<F: Float>(t8680: F, t8682: F, t8684: F, t8690: F, t8694: F, t7384: F, t7388: F, t7391: F, t7397: F, t7406: F, t8686: F, t8692: F, t8696: F, t8698: F, t8700: F) -> F {
    let t9248 = F::cast_from(11.0_f64) / F::cast_from(192.0_f64) * t8680;
    let t9249 = F::cast_from(11.0_f64) / F::cast_from(576.0_f64) * t8682;
    let t9250 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t8684;
    let t9252 = F::cast_from(0.21437009059034868486e-3_f64) * t8690;
    let t9254 = F::cast_from(0.17149607247227894789e-2_f64) * t8694;
    let t9258 = -t7384 - t7388 - t7391 + t7397 + t7406 + t9248 + t9249 + t9250 + F::cast_from(0.51448821741683684367e-2_f64) * t8686 - t9252 - F::cast_from(0.34299214494455789578e-2_f64) * t8692 + t9254 + F::cast_from(0.68598428988911579156e-2_f64) * t8696 + F::cast_from(0.17149607247227894789e-1_f64) * t8698 - F::cast_from(0.68598428988911579156e-2_f64) * t8700;
    t9258
}
