//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 764/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk764(t32385: f64, t5507: f64, t28: f64, t1286: f64, t1310: f64, t31997: f64, t32000: f64, t32002: f64, t32013: f64, t32016: f64, t32021: f64, t32025: f64, t32054: f64, t32366: f64, t32371: f64, t32375: f64, t32380: f64, t5495: f64, t5501: f64, t5504: f64, t5620: f64, t5624: f64, t7162: f64, t7168: f64, t7214: f64, t7218: f64) -> (f64, f64, f64) {
    let t32386 = t5507 * t32385;
    let t32387 = t28 * t32386;
    let t32390 = -t31997 - t32000 - t1286 * t32002 / 3.0_f64 + t5495 * t7214 / 6.0_f64 + t7162 * t5624 / 6.0_f64 + t7162 * t5620 / 6.0_f64 - t5501 * t32013 / 18.0_f64 - t32016 * t5504 / 18.0_f64 + t5501 * t32021 / 9.0_f64 - t32025 + t32054 * t1310 / 6.0_f64 + 2.0_f64 * t32366 + t5495 * t7218 / 3.0_f64 + t1286 * t32371 / 3.0_f64 + t1286 * t32375 / 6.0_f64 + t1286 * t32380 / 6.0_f64 - t5495 * t7168 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1286 * t32387;
    (t32386, t32387, t32390)
}
