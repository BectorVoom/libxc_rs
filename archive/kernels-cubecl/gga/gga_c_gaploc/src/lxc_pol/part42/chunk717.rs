//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 717/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk717<F: Float>(t14271: F, t1457: F, t12961: F, t12988: F, t13442: F, t13444: F, t13463: F, t13466: F, t13469: F, t13473: F, t13477: F, t13478: F, t13480: F, t1572: F) -> (F, F) {
    let t14340 = t1457 * t14271;
    let t14346 = -t13442 - t13444 + F::cast_from(0.38342925953920749676e1_f64) * t12961 - t13463 + F::cast_from(0.14300195980740170668e1_f64) * t1572 * t14340 + F::cast_from(0.63904876589867916127e-1_f64) * t12988 - F::cast_from(0.38342925953920749676e0_f64) * t13466 - F::cast_from(0.57514388930881124514e0_f64) * t13469 + t13473 + t13477 + t13478 + t13480;
    (t14340, t14346)
}
