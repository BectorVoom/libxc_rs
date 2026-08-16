//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 542/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk542(t332: f64, t5473: f64, t113: f64, t1273: f64, t1274: f64, t992: f64, t6: f64, t694: f64, t373: f64, t929: f64, t1095: f64, t679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5474 = t5473 * t332;
    let t5475 = t5474 * t113;
    let t5478 = t1273 * t1273;
    let t5479 = t5478 * t332;
    let t5480 = t5479 * t113;
    let t5483 = t1274 * t992;
    let t6032 = t694 * t6;
    let t6426 = t373 * t929;
    let t6757 = t679 * t1095;
    (t5474, t5475, t5478, t5479, t5480, t5483, t6032, t6426, t6757)
}
