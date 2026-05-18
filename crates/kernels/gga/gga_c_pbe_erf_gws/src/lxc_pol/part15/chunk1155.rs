//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1155/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1155<F: Float>(t13780: F, t3060: F, t3990: F, t13859: F, t1146: F, t3955: F, t1193: F, t3189: F, t9283: F, t13793: F, t14657: F, t13808: F, t4138: F) -> (F, F, F, F, F, F, F) {
    let t14705 = t3990 * t13780 * t3060;
    let t14706 = t13859 * t14705;
    let t14708 = t3955 * t1146;
    let t14710 = t1193 * t3189;
    let t14711 = t9283 * t14710;
    let t14714 = t14657 * t13793;
    let t14716 = t13808 * t4138;
    (t14705, t14706, t14708, t14710, t14711, t14714, t14716)
}
