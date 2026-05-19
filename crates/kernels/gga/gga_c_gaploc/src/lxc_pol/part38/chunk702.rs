//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 702/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk702<F: Float>(t13296: F, t600: F, t568: F, t13322: F, t531: F, t13423: F, t13424: F, t13425: F, t13428: F, t13430: F, t13436: F, t13440: F, t13442: F, t13444: F, t13446: F, t1562: F, t193: F, t557: F, t597: F) -> (F, F, F, F) {
    let t13449 = t600 * t13296;
    let t13450 = t568 * t13449;
    let t13453 = t531 * t13322;
    let t13456 = -t13423 - t13424 - t13425 - t13428 - F::cast_from(0.13803453343411469884e2_f64) * t1562 * t13430 + t13436 - t13440 - t13442 - t13444 + F::cast_from(0.35750489951850426669e0_f64) * t13446 * t193 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t13450 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t13453;
    (t13449, t13450, t13453, t13456)
}
