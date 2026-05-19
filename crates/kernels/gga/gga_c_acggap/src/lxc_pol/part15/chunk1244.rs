//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1244/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1244<F: Float>(t31241: F, t35425: F, t35456: F, t35471: F, t37551: F, t37555: F, t37557: F, t37560: F, t37564: F, t39962: F, t39965: F, t39967: F, t39969: F, t39971: F, t39973: F, t39977: F, t39981: F) -> F {
    let t41882 = F::cast_from(0.19055119163586549766e-1_f64) * t35425 - F::cast_from(0.83861579438944405516e-3_f64) * t31241 - t37551 + t37555 - t37557 - F::cast_from(0.51448821741683684367e-2_f64) * t39962 + F::cast_from(0.42874018118069736972e-2_f64) * t35456 + t37560 - t37564 + F::cast_from(0.51448821741683684367e-2_f64) * t39965 + F::cast_from(0.38110238327173099531e-2_f64) * t35471 - F::cast_from(0.68598428988911579156e-2_f64) * t39967 + F::cast_from(0.34299214494455789578e-2_f64) * t39969 - F::cast_from(0.34299214494455789578e-2_f64) * t39971 + F::cast_from(0.17149607247227894789e-2_f64) * t39973 + F::cast_from(0.8386157943894440552e-3_f64) * t39977 + F::cast_from(0.85748036236139473944e-3_f64) * t39981;
    t41882
}
