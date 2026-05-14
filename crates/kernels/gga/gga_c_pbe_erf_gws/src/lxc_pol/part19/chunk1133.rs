//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1133/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1133<F: Float>(t15161: F, t2397: F, t12074: F, t3079: F, t14135: F, t3912: F, t51913: F, t11505: F, t3972: F, t3975: F, t15288: F, t1134: F, t13796: F, t13859: F, t3097: F, t1113: F, t814: F, t9847: F) -> (F, F, F, F, F, F, F) {
    let t56588 = t15161 * t2397;
    let t56590 = t12074 * t3079;
    let t56593 = t3912 * t14135 * t51913;
    let t56596 = t3972 * t3975 * t11505;
    let t56599 = t15288 * t2397;
    let t56604 = t13859 * t13796 * t3097 * t1134;
    let t56613 = t3972 * t3975 * t1113 * t9847 * t814;
    (t56588, t56590, t56593, t56596, t56599, t56604, t56613)
}
