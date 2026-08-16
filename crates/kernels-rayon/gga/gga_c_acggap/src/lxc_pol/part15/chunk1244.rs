//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1244/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1244(t31241: f64, t35425: f64, t35456: f64, t35471: f64, t37551: f64, t37555: f64, t37557: f64, t37560: f64, t37564: f64, t39962: f64, t39965: f64, t39967: f64, t39969: f64, t39971: f64, t39973: f64, t39977: f64, t39981: f64) -> f64 {
    let t41882 = 0.19055119163586549766e-1_f64 * t35425 - 0.83861579438944405516e-3_f64 * t31241 - t37551 + t37555 - t37557 - 0.51448821741683684367e-2_f64 * t39962 + 0.42874018118069736972e-2_f64 * t35456 + t37560 - t37564 + 0.51448821741683684367e-2_f64 * t39965 + 0.38110238327173099531e-2_f64 * t35471 - 0.68598428988911579156e-2_f64 * t39967 + 0.34299214494455789578e-2_f64 * t39969 - 0.34299214494455789578e-2_f64 * t39971 + 0.17149607247227894789e-2_f64 * t39973 + 0.8386157943894440552e-3_f64 * t39977 + 0.85748036236139473944e-3_f64 * t39981;
    t41882
}
