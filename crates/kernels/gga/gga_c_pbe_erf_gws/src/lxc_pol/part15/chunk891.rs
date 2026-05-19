//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 891/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk891<F: Float>(t256: F, t7733: F, t1918: F, t2654: F, t5384: F, t5387: F, t5388: F, t7689: F, t7693: F, t7697: F, t7702: F, t7708: F, t7710: F, t7712: F, t7715: F, t7719: F, t7724: F, t7728: F, t7732: F) -> F {
    let t7734 = t7733 * t256;
    let t7736 = t2654 * t1918;
    let t7738 = t7689 + t7693 - t7697 + t7702 + t7708 + t7710 - t7712 - t7715 + t7719 - t7724 - t5384 + t5387 + F::new(2.0) / F::new(9.0) * t5388 + t7728 + t7732 + t7734 / F::new(3.0) + F::cast_from(0.12155555555555555555e0_f64) * t7736;
    t7738
}
