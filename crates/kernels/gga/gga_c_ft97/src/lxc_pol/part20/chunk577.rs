//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 577/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk577<F: Float>(t1882: F, t2869: F, t8232: F, t837: F, t877: F, t2834: F, t681: F, t89: F, t313: F, t9555: F, t2811: F, t2807: F, t295: F, t9568: F, t2803: F, t842: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10730 = t1882 * t2869;
    let t10732 = t8232 * t837;
    let t10735 = t8232 * t877;
    let t10745 = t89 * t681 * t2834;
    let t10749 = 28.0 / 81.0 * t89 * t9555 * t313;
    let t10750 = t1882 * t2811;
    let t10752 = t1882 * t2807;
    let t10758 = t9568 * t295;
    let t10771 = t1882 * t2803;
    let t10773 = t8232 * t842;
    (t10730, t10732, t10735, t10745, t10749, t10750, t10752, t10758, t10771, t10773)
}
