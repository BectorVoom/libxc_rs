//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 848/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk848<F: Float>(t8230: F, t8254: F, t138: F, t1572: F, t1577: F, t1578: F, t1590: F, t2902: F, t2905: F, t2919: F, t514: F, t520: F, t5844: F, t5847: F, t5854: F, t8204: F, t8206: F, t8209: F, t8218: F, t8221: F, t8224: F, t985: F) -> (F,) {
    let t8255 = t8230 + t8254;
    let t8257 = t138 * t8204 - 2.0 * t1572 * t2919 + 4.0 * t1577 * t8221 + 2.0 * t1577 * t8224 + 2.0 * t1578 * t8209 - t1590 * t2902 + 4.0 * t2905 * t5847 - t514 * t8255 - 2.0 * t520 * t8206 - t5844 * t985 - 6.0 * t5854 * t8218;
    (t8257,)
}
