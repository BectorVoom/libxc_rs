//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 874/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk874<F: Float>(t837: F, t991: F, t551: F, t553: F, t1052: F, t163: F, t169: F, t784: F, t171: F, t5891: F, t5895: F, t5898: F, t6003: F, t6005: F, t6008: F, t6012: F, t6015: F, t6018: F, t6021: F, t8385: F, t8387: F, t8390: F, t8395: F, t8460: F) -> (F,) {
    let t8465 = t837 * t991;
    let t8467 = t8465 * t551 * t553;
    let t8471 = t169 * t784 * t1052 * t163;
    let t8473 = -t8385 - 0.39507780657818961764e-2 * t8387 - 0.49542756944904978052e-3 * t8390 + t8395 - t6005 + 0.13169260219272987255e-1 * t6008 + t6012 + t6015 - 0.19753890328909480882e-2 * t6018 - 0.79015561315637923528e-2 * t6021 - 0.99085513889809956104e-3 * t5891 - t5895 - t5898 + t6003 - 0.53884053046145740922e-2 * t169 * t171 * t8460 * t163 + 0.65846301096364936273e-2 * t8467 - 0.23948468020509218188e-1 * t8471;
    (t8473,)
}
