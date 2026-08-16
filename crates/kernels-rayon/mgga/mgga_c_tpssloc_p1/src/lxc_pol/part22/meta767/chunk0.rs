//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2592/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2592(t1227: f64, t21776: f64, t248: f64, t3521: f64, t18392: f64, t5005: f64, t15737: f64, t18356: f64, t19040: f64, t5024: f64, t11738: f64, t22299: f64, t3570: f64) -> (f64, f64, f64, f64, f64) {
    let t72273 = t1227 * t248 * t3521 * t21776;
    let t72285 = t5005 * t18392;
    let t72287 = t15737 * t18356;
    let t72289 = t5024 * t19040;
    let t72293 = t11738 * t248 * t3570 * t22299;
    (t72273, t72285, t72287, t72289, t72293)
}
