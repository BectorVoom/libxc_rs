//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 969/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk969(t31316: f64, t6547: f64, t112663: f64, t112666: f64, t112668: f64, t112672: f64, t112674: f64, t112676: f64, t112679: f64, t112681: f64, t112685: f64, t114592: f64, t114596: f64, t114599: f64, t114604: f64, t114606: f64, t114610: f64, t114613: f64) -> f64 {
    let t114615 = t6547 * t31316;
    let t114617 = -t112663 - 0.16449340668482264365e-1_f64 * t114592 - 0.49348022005446793095e-1_f64 * t114596 + 0.16449340668482264365e-1_f64 * t114599 + 0.3289868133696452873e-1_f64 * t114604 - t112666 + t112668 - 0.76763589786250567036e-1_f64 * t114606 - t112672 + 0.16449340668482264365e-1_f64 * t114610 - 0.16449340668482264365e-1_f64 * t114613 + t112674 - t112676 - 0.38381794893125283518e-1_f64 * t114615 + t112679 - t112681 - t112685;
    t114617
}
