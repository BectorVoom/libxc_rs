//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 974/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk974<F: Float>(t536: F, t5975: F, t6: F, t6045: F, t153: F, t413: F, t7236: F, t7271: F, t161: F, t148: F, t163: F, t16580: F, t41: F) -> (F, F, F, F, F, F) {
    let t18041 = t536 * t5975;
    let t18046 = t6 * t6045;
    let t18049 = F::new(0.17888888888888888889e-1) * t7271 + F::new(0.22252592592592592592e0) * t7236 - F::new(0.7316671043820612376e-1) * t413 + F::new(0.15663796296296296297e-1) * t153 * t18046;
    let t18050 = t18049 * t161;
    let t18053 = F::new(0.31505407223141117834e-1) * t148 * t18050 * t163;
    let t18054 = t41 * t16580;
    (t18041, t18046, t18049, t18050, t18053, t18054)
}
