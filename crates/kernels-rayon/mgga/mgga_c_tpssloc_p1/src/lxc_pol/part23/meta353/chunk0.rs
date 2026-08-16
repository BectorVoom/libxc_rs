//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1148/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1148(t23076: f64, t241: f64, t67: f64, t2559: f64, t2570: f64, t782: f64, t9558: f64, t786: f64, t9569: f64, t222: f64, t39934: f64, t2691: f64, t812: f64, t815: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40971 = t241 * t23076 * t67;
    let t41008 = t2559 * t2570;
    let t41011 = t782 * t9558;
    let t41083 = t9569 * t786;
    let t41096 = 455.0_f64 / 243.0_f64 * t39934 * t222;
    let t41115 = t812 * t815 * t2691;
    (t40971, t41008, t41011, t41083, t41096, t41115)
}
