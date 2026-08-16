//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1184/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1184(t2031: f64, t83718: f64, t2240: f64, t240: f64, t33: f64, t6492: f64, t2244: f64, t63: f64, t23993: f64, t6495: f64, t2032: f64, t22493: f64, t22537: f64, t23963: f64, t24001: f64, t6486: f64, t7035: f64, t83717: f64, t83734: f64, t83748: f64, t83822: f64) -> f64 {
    let t84237 = t2031 * t83718;
    let t84241 = t2240 * t33 * t240;
    let t84242 = t84241 * t6492;
    let t84245 = t2240 * t2244 * t63;
    let t84248 = t6495 * t23993;
    let t84258 = t22493 * t7035 + t6486 * t24001 + 30.0_f64 * t23963 * t83734 - 60.0_f64 * t83717 * t84237 - 440.0_f64 / 9.0_f64 * t84242 + 10.0_f64 * t84245 * t6492 - 176.0_f64 / 9.0_f64 * t84248 - 2.0_f64 / 3.0_f64 * t83822 * t2032 - 2.0_f64 * t22537 * t7035 - 2.0_f64 * t6495 * t24001 - 2.0_f64 * t83748 * t2032;
    t84258
}
