//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 931/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk931(t5311: f64, t8232: f64, t5327: f64, t5381: f64, t2770: f64, t5374: f64, t5332: f64, t38953: f64, t5415: f64, t2399: f64, t5376: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t71532 = t8232 * t5311;
    let t71534 = t8232 * t5327;
    let t71589 = t8232 * t5381;
    let t71624 = t2770 * t5374;
    let t71630 = t8232 * t5332;
    let t71846 = t38953 * t5415;
    let t71907 = t89 * t2399 * t5376;
    (t71532, t71534, t71589, t71624, t71630, t71846, t71907)
}
