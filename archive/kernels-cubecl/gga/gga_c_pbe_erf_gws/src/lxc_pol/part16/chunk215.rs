//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 215/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk215<F: Float>(t220: F, t617: F, t186: F, t616: F, t174: F, t205: F, t567: F, t213: F, t56: F) -> (F, F, F, F, F, F) {
    let t618 = t220 * t617;
    let t619 = t186 * t618;
    let t621 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t616 * t619;
    let t623 = t174 * t567 * t205;
    let t624 = F::cast_from(0.18891666666666666667e-2_f64) * t623;
    let t625 = t56 * t213;
    (t618, t619, t621, t623, t624, t625)
}
