//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 978/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk978<F: Float>(t1162: F, t810: F, t353: F, t4386: F, t1118: F, t814: F, t3037: F, t328: F, t2306: F, t3074: F, t2501: F, t2370: F, t830: F) -> (F, F, F, F, F, F) {
    let t8693 = t1162 * t810;
    let t8694 = t353 * t8693;
    let t8695 = t4386 * t8694;
    let t8698 = t1118 * t814;
    let t8699 = t353 * t8698;
    let t8700 = t4386 * t8699;
    let t8703 = t3037 * t328;
    let t8704 = t2306 * t8703;
    let t8705 = t3074 * t8704;
    let t8708 = t2501 * t810;
    let t8710 = t2370 * t830 * t8708;
    (t8695, t8700, t8703, t8705, t8708, t8710)
}
