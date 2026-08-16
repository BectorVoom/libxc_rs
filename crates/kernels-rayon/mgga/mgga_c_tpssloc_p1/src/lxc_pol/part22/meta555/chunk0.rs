//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2055/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2055(t2559: f64, t2570: f64, t782: f64, t9558: f64, t2617: f64, t9600: f64, t786: f64, t9569: f64, t805: f64, t222: f64, t39934: f64, t9637: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41008 = t2559 * t2570;
    let t41011 = t782 * t9558;
    let t41052 = t2617 * t9600;
    let t41083 = t9569 * t786;
    let t41084 = t41083 * t805;
    let t41096 = 455.0_f64 / 243.0_f64 * t39934 * t222;
    let t41107 = t2617 * t9637;
    (t41008, t41011, t41052, t41083, t41084, t41096, t41107)
}
