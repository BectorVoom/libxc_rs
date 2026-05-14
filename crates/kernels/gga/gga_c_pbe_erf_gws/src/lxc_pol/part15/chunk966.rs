//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 966/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk966<F: Float>(t1176: F, t2298: F, t367: F, t1178: F, t2402: F, t371: F, t4052: F, t810: F, t2376: F, t2409: F, t1192: F, t2352: F, t3067: F, t4007: F, t6781: F, t2417: F) -> (F, F, F, F, F, F, F, F) {
    let t13830 = t1176 * t367 * t2298;
    let t13832 = t371 * t1178 * t2402;
    let t13833 = t13830 * t13832;
    let t13835 = t4052 * t810;
    let t13837 = t2409 * t2376 * t13835;
    let t13840 = t1192 * t2352;
    let t13842 = t2409 * t3067 * t13840;
    let t13846 = t2409 * t6781 * t4007;
    let t13849 = t1192 * t2417;
    (t13832, t13833, t13835, t13837, t13840, t13842, t13846, t13849)
}
