//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1005/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1005(t11159: f64, t164: f64, t11187: f64, t11240: f64, t163: f64, t169: f64, t171: f64, t5891: f64, t5895: f64, t5898: f64, t6003: f64, t6005: f64, t6008: f64, t6012: f64, t6015: f64, t6021: f64, t8385: f64, t8387: f64, t8390: f64, t8395: f64, t8467: f64) -> f64 {
    let t11250 = t11159 * t164;
    let t11253 = 0.89806755076909568204e-2_f64 * t11187 - 0.53884053046145740922e-2_f64 * t169 * t171 * t11240 * t163 - t8385 - 0.79015561315637923528e-2_f64 * t8387 - 0.99085513889809956104e-3_f64 * t8390 + t8395 - t6005 + 0.65846301096364936273e-2_f64 * t6008 + t6012 + t6015 - 0.39507780657818961764e-2_f64 * t6021 - 0.49542756944904978052e-3_f64 * t5891 - t5895 - t5898 + t6003 + 0.31505407223141117834e-1_f64 * t11250 + 0.13169260219272987255e-1_f64 * t8467;
    t11253
}
