//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2019/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2019(t591: f64, t9688: f64, t2386: f64, t240: f64, t2385: f64, t2558: f64, t686: f64, t685: f64, t120: f64, t118: f64, t123: f64, t116: f64, t268: f64, t8705: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39275 = t9688 * t591;
    let t39277 = t2386 * t240;
    let t39278 = t2385 * t39277;
    let t39280 = t686 * t2558;
    let t39281 = t685 * t39280;
    let t39283 = t120 * t2558;
    let t39284 = t118 * t39283;
    let t39286 = f64::powf(t123, -0.25e1_f64);
    let t39289 = t39286 * t116 * t8705 * t268;
    (t39275, t39277, t39278, t39280, t39281, t39283, t39284, t39289)
}
