//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1035/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1035<F: Float>(t3754: F, t6203: F, t3912: F, t6252: F, t2150: F, t3841: F, t6542: F, t2255: F, t2313: F, t3781: F, t339: F, t3780: F) -> (F, F, F, F, F) {
    let t11598 = t6203 * t3754;
    let t11600 = t3912 * t6252;
    let t11602 = t11600 * t2150 / F::new(48.0);
    let t11603 = t6542 * t3841;
    let t11604 = F::new(7.0) / F::new(144.0) * t11603;
    let t11606 = t2255 * t3781 * t2313;
    let t11609 = t3780 * t339;
    (t11598, t11602, t11604, t11606, t11609)
}
