//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 858/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk858<F: Float>(t353: F, t8688: F, t859: F, t1162: F, t810: F, t4386: F, t1118: F, t814: F, t2501: F, t2370: F, t830: F, t1105: F, t898: F) -> (F, F, F, F, F, F) {
    let t8689 = t353 * t8688;
    let t8690 = t859 * t8689;
    let t8693 = t1162 * t810;
    let t8694 = t353 * t8693;
    let t8695 = t4386 * t8694;
    let t8698 = t1118 * t814;
    let t8699 = t353 * t8698;
    let t8700 = t4386 * t8699;
    let t8708 = t2501 * t810;
    let t8710 = t2370 * t830 * t8708;
    let t8713 = t898 * t1105;
    (t8690, t8695, t8700, t8708, t8710, t8713)
}
