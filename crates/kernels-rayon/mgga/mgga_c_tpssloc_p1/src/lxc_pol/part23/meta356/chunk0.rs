//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1153/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1153(t41654: f64, t10969: f64, t154: f64, t2769: f64, t2289: f64, t2903: f64, t2928: f64, t315: f64, t10213: f64, t241: f64, t270: f64, t276: f64, t39267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41655 = 0.18467901234567901234e0_f64 * t41654;
    let t41664 = t154 * t10969;
    let t41665 = t2769 * t2769;
    let t41666 = 1.0_f64 / t41665;
    let t41687 = 1.0_f64 / t2769 / t2289;
    let t41741 = 0.96141975308641975307e-1_f64 * t41654;
    let t41825 = 1.0_f64 / t2928 / t2903;
    let t41826 = t315 * t41825;
    let t41880 = t241 * t10213;
    let t41904 = 280.0_f64 / 81.0_f64 * t41654;
    let t41935 = 1.0_f64 / t276 / t39267 / t270 / 96.0_f64;
    (t41655, t41664, t41666, t41687, t41741, t41825, t41826, t41880, t41904, t41935)
}
