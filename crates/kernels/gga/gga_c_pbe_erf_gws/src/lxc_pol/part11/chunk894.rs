//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 894/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk894<F: Float>(t163: F, t169: F, t234: F, t922: F, t1: F, t4576: F, t550: F, t553: F, t6: F, t6045: F, t153: F, t413: F, t7236: F, t7271: F) -> (F, F, F, F, F) {
    let t18021 = F::new(0.40978489723982440011e0) * t169 * t922 * t234 * t163;
    let t18032 = t4576 * t1;
    let t18035 = F::new(0.79015561315637923528e-2) * t550 * t18032 * t553;
    let t18046 = t6 * t6045;
    let t18049 = F::new(0.17888888888888888889e-1) * t7271 + F::new(0.22252592592592592592e0) * t7236 - F::new(0.7316671043820612376e-1) * t413 + F::new(0.15663796296296296297e-1) * t153 * t18046;
    (t18021, t18032, t18035, t18046, t18049)
}
