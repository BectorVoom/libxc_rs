//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1464/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1464(t26504: f64, t8607: f64, t120172: f64, t120719: f64, t120721: f64, t120728: f64, t120730: f64, t120735: f64, t122706: f64, t122708: f64, t122710: f64, t122713: f64, t122732: f64, t122734: f64, t122735: f64, t122736: f64, t122737: f64, t122738: f64, t122739: f64, t122740: f64, t122754: f64, t1774: f64, t2039: f64, t24999: f64, t26875: f64, t31700: f64, t5107: f64, t574: f64, t7056: f64, t8519: f64, t90400: f64, t96361: f64) -> f64 {
    let t122758 = t8607 * t26504;
    let t122761 = 6.0_f64 * t120172 * t26875 - t120719 - t120721 - t120728 - t120730 - t120735 - t122706 - t122708 - t122710 - t122713 + (2.0_f64 * t2039 * t90400 + 2.0_f64 * t2039 * t96361 + 2.0_f64 * t24999 * t7056 + 2.0_f64 * t122732 + 2.0_f64 * t122734 + 2.0_f64 * t122735 + 2.0_f64 * t122736 + 2.0_f64 * t122737 + 2.0_f64 * t122738 + 2.0_f64 * t122739 + 2.0_f64 * t122740 + t122754) * t574 + t122758 - t31700 * t1774 - t8519 * t5107;
    t122761
}
