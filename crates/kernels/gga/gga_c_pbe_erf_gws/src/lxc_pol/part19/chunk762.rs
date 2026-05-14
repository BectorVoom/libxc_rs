//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 762/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk762<F: Float>(t2650: F, t723: F, t1022: F, t5212: F, t108: F, t210: F, t267: F, t1791: F, t641: F, t1018: F, t1672: F, t185: F, t2789: F, t586: F, t2659: F, t2816: F, t636: F) -> (F, F, F, F, F, F, F, F) {
    let t7075 = t2650 * t723;
    let t7106 = t5212 * t1022;
    let t7114 = t210 * t108;
    let t7115 = t7114 * t267;
    let t7116 = t641 * t1791;
    let t7121 = t1672 * t1018;
    let t7122 = t185 * t7121;
    let t7130 = t2789 * t586;
    let t7136 = t2659 * t586;
    let t7147 = 8.0 / 45.0 * t2816 * t636;
    (t7075, t7106, t7115, t7116, t7122, t7130, t7136, t7147)
}
