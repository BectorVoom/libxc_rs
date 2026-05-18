//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 631/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk631<F: Float>(t637: F, t639: F, t8709: F, t2253: F, t2296: F, t3626: F, t70: F, t170: F, t180: F, t645: F, t8640: F, t2265: F, t631: F, t8662: F, t8665: F, t8669: F, t8672: F, t8676: F, t8678: F, t8682: F, t8686: F) -> (F, F, F) {
    let t8711 = t637 * t639 * t8709;
    let t8714 = t2253 * t2296;
    let t8715 = t3626 * t70;
    let t8718 = F::new(20.0) / F::new(27.0) * t170 * t8715 * t180;
    let t8719 = t8640 * t645;
    let t8721 = -t631 * t8662 / F::new(3.0) + t2265 * t8665 / F::new(6.0) - t2265 * t8669 - t2265 * t8672 + F::new(4.0) / F::new(3.0) * t8676 + F::new(2.0) / F::new(3.0) * t8678 + F::new(3.0) * t2265 * t8682 + F::new(2.0) * t2265 * t8686 + t631 * t8711 / F::new(2.0) - t8714 + t8718 + F::new(5.0) / F::new(3.0) * t8719;
    (t8711, t8715, t8721)
}
