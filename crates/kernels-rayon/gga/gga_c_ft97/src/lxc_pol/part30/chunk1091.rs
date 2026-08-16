//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1091/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1091(t10697: f64, t4299: f64, t7672: f64, t112390: f64, t6374: f64, t34281: f64, t6963: f64, t1466: f64, t36016: f64, t681: f64, t1091: f64, t111711: f64, t142501: f64, t142503: f64, t142595: f64, t142597: f64, t142950: f64, t193: f64, t25459: f64, t2665: f64, t29416: f64, t34322: f64, t35810: f64, t6216: f64, t6222: f64, t7614: f64) -> (f64, f64, f64) {
    let t152631 = t10697 * t7672 * t4299;
    let t152633 = t112390 * t6374;
    let t152635 = t6963 * t34281;
    let t152638 = t1466 * t681 * t36016;
    let t152644 = -t6216 * t2665 * t142950 * t1091 / 18.0_f64 + t29416 * t7614 / 6.0_f64 - 2.0_f64 / 3.0_f64 * t1466 * t193 * t6222 * t111711 - t142501 / 18.0_f64 - t142503 / 18.0_f64 + t6963 * t34322 / 6.0_f64 - 12.0_f64 * t152631 + 8.0_f64 * t152633 + t152635 / 9.0_f64 - t152638 / 18.0_f64 - t25459 * t35810 / 18.0_f64 + t142595 / 54.0_f64 + t142597 / 27.0_f64;
    (t152631, t152633, t152644)
}
