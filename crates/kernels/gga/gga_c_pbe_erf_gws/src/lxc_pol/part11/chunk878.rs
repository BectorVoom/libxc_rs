//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 878/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk878<F: Float>(t1105: F, t3718: F, t2429: F, t1168: F, t3703: F, t1167: F, t3931: F, t6854: F, t321: F, t3932: F, t804: F, t1109: F, t2118: F) -> (F, F, F, F, F, F) {
    let t13713 = t3718 * t1105;
    let t13714 = t2429 * t13713;
    let t13716 = t1168 * t3703;
    let t13717 = t2429 * t13716;
    let t13719 = t3931 * t1167;
    let t13720 = t13719 * t6854;
    let t13721 = t321 * t13720;
    let t13726 = t804 * t3932 * t1105;
    let t15149 = t2118 * t1109;
    (t13714, t13717, t13720, t13721, t13726, t15149)
}
