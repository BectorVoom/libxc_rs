//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1060/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1060<F: Float>(t281: F, t285: F, t4576: F, t535: F, t147: F, t18049: F, t520: F, t5621: F, t5624: F, t159: F, t18068: F, t545: F, t5984: F) -> (F, F, F, F, F) {
    let t19152 = t281 * t535 * t4576 * t285;
    let t19157 = F::new(0.11974234010254609094e-1) * t281 * t147 * t18049 * t285;
    let t19160 = t5621 * t520;
    let t19161 = t19160 * t5624;
    let t19165 = t18068 * t159 * t285;
    let t19169 = F::new(0.26861343269868796571e-1) * t5984 * t545 * t285;
    (t19152, t19157, t19161, t19165, t19169)
}
