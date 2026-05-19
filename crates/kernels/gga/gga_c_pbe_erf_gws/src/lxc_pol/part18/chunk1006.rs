//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1006/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1006<F: Float>(t11159: F, t164: F, t11187: F, t11240: F, t163: F, t169: F, t171: F, t5891: F, t5895: F, t5898: F, t6003: F, t6005: F, t6008: F, t6012: F, t6015: F, t6021: F, t8385: F, t8387: F, t8390: F, t8395: F, t8467: F) -> F {
    let t11250 = t11159 * t164;
    let t11253 = F::cast_from(0.89806755076909568204e-2_f64) * t11187 - F::cast_from(0.53884053046145740922e-2_f64) * t169 * t171 * t11240 * t163 - t8385 - F::cast_from(0.79015561315637923528e-2_f64) * t8387 - F::cast_from(0.99085513889809956104e-3_f64) * t8390 + t8395 - t6005 + F::cast_from(0.65846301096364936273e-2_f64) * t6008 + t6012 + t6015 - F::cast_from(0.39507780657818961764e-2_f64) * t6021 - F::cast_from(0.49542756944904978052e-3_f64) * t5891 - t5895 - t5898 + t6003 + F::cast_from(0.31505407223141117834e-1_f64) * t11250 + F::cast_from(0.13169260219272987255e-1_f64) * t8467;
    t11253
}
