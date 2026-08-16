//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 975/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk975<F: Float>(t4563: F, t547: F, t163: F, t169: F, t299: F, t5962: F, t366: F, t684: F, t5985: F, t413: F, t535: F, t164: F) -> (F, F, F, F, F, F) {
    let t18057 = t4563 * t547;
    let t18061 = t169 * t299 * t5962 * t163;
    let t18065 = t169 * t366 * t684 * t163;
    let t18067 = F::cast_from(0.756129773355386828e0_f64) * t5985;
    let t18068 = t413 * t535;
    let t18069 = t18068 * t164;
    (t18057, t18061, t18065, t18067, t18068, t18069)
}
