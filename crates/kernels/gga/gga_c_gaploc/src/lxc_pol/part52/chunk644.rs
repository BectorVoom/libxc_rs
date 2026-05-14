//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 644/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk644<F: Float>(t14271: F, t1457: F, t12961: F, t12988: F, t13442: F, t13444: F, t13463: F, t13466: F, t13469: F, t13473: F, t13477: F, t13478: F, t13480: F, t1572: F, t14313: F, t14326: F, t14338: F) -> (F, F) {
    let t14340 = t1457 * t14271;
    let t14346 = -t13442 - t13444 + 0.38342925953920749676e1 * t12961 - t13463 + 0.14300195980740170668e1 * t1572 * t14340 + 0.63904876589867916127e-1 * t12988 - 0.38342925953920749676e0 * t13466 - 0.57514388930881124514e0 * t13469 + t13473 + t13477 + t13478 + t13480;
    let t14348 = t14313 + t14326 + t14338 + t14346;
    (t14340, t14348)
}
