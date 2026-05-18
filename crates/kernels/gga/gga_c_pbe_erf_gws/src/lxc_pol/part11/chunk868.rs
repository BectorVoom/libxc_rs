//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 868/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk868<F: Float>(t13612: F, t6126: F, t338: F, t353: F, t1144: F, t3722: F, t1161: F, t3886: F, t2409: F, t3067: F, t3887: F, t1118: F, t3907: F) -> (F, F, F, F, F, F, F) {
    let t13613 = t6126 * t13612;
    let t13615 = t338 * t353 * t13613;
    let t13619 = t338 * t1144 * t3722;
    let t13622 = t1161 * t3886;
    let t13624 = t2409 * t3067 * t13622;
    let t13628 = t338 * t1144 * t3887;
    let t13635 = t338 * t3907 * t1118;
    (t13613, t13615, t13619, t13622, t13624, t13628, t13635)
}
